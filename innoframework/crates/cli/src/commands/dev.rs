//! `inno dev` — serves the built web output with the generated CSP header.
//!
//! Serves `target/dx/<app>/release/web/public` (produced by `inno build web`)
//! on 127.0.0.1 and attaches the `Content-Security-Policy` value from
//! `csp-header.txt` to every response (CSP-1, ADR-04). Standard library only.

use crate::errors::{CliErrorCode, InnoframeworkCliError};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};

/// Run the static server for the built web output.
pub fn run(port: u16) -> Result<(), InnoframeworkCliError> {
    if !Path::new("inno.toml").exists() {
        return Err(InnoframeworkCliError::build_no_manifest());
    }

    let public = find_public_dir().ok_or_else(|| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            "no web build output found",
            "run `inno build web` first",
            Some("https://innoframework.dev/docs/cli/dev#no-build"),
        )
    })?;

    let csp = std::fs::read_to_string(public.join("csp-header.txt")).map_err(|_| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            "csp-header.txt is missing from the build output",
            "re-run `inno build web` to regenerate it",
            None,
        )
    })?;
    let csp = csp.trim().to_string();

    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|_| InnoframeworkCliError::dev_port_in_use(port))?;
    println!(
        "inno dev: serving {} on http://127.0.0.1:{port}",
        public.display()
    );
    println!("Content-Security-Policy applied to every response. Press Ctrl+C to stop.");

    for stream in listener.incoming().flatten() {
        // Errors on one connection must not stop the server.
        let _ = handle(stream, &public, &csp);
    }
    Ok(())
}

/// Locate the dx web output directory under `target/dx`.
pub fn find_public_dir_in(root: &Path) -> Option<PathBuf> {
    for entry in std::fs::read_dir(root).ok()?.flatten() {
        let candidate = entry.path().join("release").join("web").join("public");
        if candidate.join("index.html").exists() {
            return Some(candidate);
        }
    }
    None
}

fn find_public_dir() -> Option<PathBuf> {
    find_public_dir_in(Path::new("target/dx"))
}

fn handle(mut stream: std::net::TcpStream, public: &Path, csp: &str) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf)?;
    let request = String::from_utf8_lossy(&buf[..n]);
    let target = request
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .unwrap_or("/");

    let path_part = target.split('?').next().unwrap_or("/");
    let rel = if path_part == "/" {
        "index.html"
    } else {
        path_part.trim_start_matches('/')
    };

    // Reject traversal: only serve files that resolve inside the public dir.
    let file = public.join(rel);
    let inside = file
        .canonicalize()
        .ok()
        .zip(public.canonicalize().ok())
        .map(|(f, root)| f.starts_with(root))
        .unwrap_or(false);

    let (status, content_type, body) = if inside && file.is_file() {
        let body = std::fs::read(&file)?;
        ("200 OK", content_type(&file), body)
    } else {
        (
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"not found".to_vec(),
        )
    };

    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nContent-Security-Policy: {csp}\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(&body)?;
    Ok(())
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
