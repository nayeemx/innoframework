//! `inno dev` — runs the dev server for a local project.

use crate::errors::InnoframeworkCliError;

/// Run the dev server on the given port.
pub fn run(port: u16) -> Result<(), InnoframeworkCliError> {
    // Check for inno.toml manifest
    let manifest_path = std::path::Path::new("inno.toml");
    if !manifest_path.exists() {
        return Err(crate::errors::InnoframeworkCliError::build_no_manifest());
    }

    // Check if port is available (basic check by trying to bind)
    use std::net::TcpListener;
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => {
            // Port is available, listener dropped immediately
        }
        Err(_) => {
            return Err(crate::errors::InnoframeworkCliError::dev_port_in_use(port));
        }
    }

    println!("inno dev: serving on http://127.0.0.1:{port}");
    println!("Press Ctrl+C to stop.");

    // Read manifest to determine targets
    let manifest = std::fs::read_to_string(manifest_path).map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::BuildNoManifest,
            format!("cannot read inno.toml: {e}"),
            "check file permissions",
            None,
        )
    })?;

    if manifest.contains("targets = [") && manifest.contains("\"web\"") {
        println!("Detected web target. Run `inno build web` to build, then serve with a static file server.");
    }

    // Dev server: for now, just watch and report. A full dev server with
    // live-reload comes after the web build target works.
    println!("Dev server ready. Edit files and re-run `inno build web` to rebuild.");
    println!("(Live-reload and file watching are P4+ features.)");

    Ok(())
}
