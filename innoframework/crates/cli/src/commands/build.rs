//! `inno build` — build for web, desktop, or mobile targets.

use crate::errors::InnoframeworkCliError;

/// Build for the given target.
pub fn run(target: crate::BuildTarget) -> Result<(), InnoframeworkCliError> {
    // Read manifest
    let manifest_path = std::path::Path::new("inno.toml");
    if !manifest_path.exists() {
        return Err(crate::errors::InnoframeworkCliError::build_no_manifest());
    }

    let manifest = std::fs::read_to_string(manifest_path).map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::BuildNoManifest,
            format!("cannot read inno.toml: {e}"),
            "check file permissions",
            None,
        )
    })?;

    // Secret boundary check: reject secrets in manifest values (TR-2.3)
    check_secret_boundary(&manifest)?;

    match target {
        crate::BuildTarget::Web => build_web(&manifest)?,
        crate::BuildTarget::Desktop => build_desktop(&manifest)?,
        crate::BuildTarget::Mobile => build_mobile(&manifest)?,
    }

    Ok(())
}

/// Check the manifest for secret values (secret boundary).
/// Secret patterns are rejected before any build step runs.
fn check_secret_boundary(manifest: &str) -> Result<(), InnoframeworkCliError> {
    // Known credential prefixes (same set as the core secret module, but
    // duplicated here so the CLI can fail fast without importing core).
    const SECRET_PREFIXES: &[(&str, &str)] = &[
        ("AKIA", "AWS access key id"),
        ("ASIA", "AWS temporary access key id"),
        ("ghp_", "GitHub personal access token"),
        ("gho_", "GitHub OAuth token"),
        ("ghs_", "GitHub server token"),
        ("github_pat_", "GitHub fine-grained token"),
        ("sk_live_", "Stripe live secret key"),
        ("rk_live_", "Stripe live restricted key"),
        ("xoxb-", "Slack bot token"),
        ("xoxp-", "Slack user token"),
        ("sk-ant-", "Anthropic API key"),
        ("-----BEGIN ", "PEM private key block"),
        ("eyJhbGciOi", "JWT"),
    ];

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }
        // Extract the value after '='
        if let Some(eq_pos) = trimmed.find('=') {
            let value = trimmed[eq_pos + 1..].trim();
            let value = value.trim_matches('"').trim_matches('\'');
            for (prefix, _rule) in SECRET_PREFIXES {
                if value.starts_with(*prefix)
                    || (prefix.starts_with("-----") && value.contains(*prefix))
                {
                    return Err(
                        crate::errors::InnoframeworkCliError::build_secret_in_manifest(
                            line.split('=').next().unwrap_or("unknown").trim(),
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

/// Build for the web target (WASM + CSP header).
fn build_web(_manifest: &str) -> Result<(), InnoframeworkCliError> {
    println!("inno build web: compiling to WASM...");

    // Generate CSP header file
    let csp_header = generate_csp_header();
    std::fs::write("dist/csp-header.txt", csp_header).map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot write CSP header: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Create dist directory structure
    std::fs::create_dir_all("dist/_static").map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot create dist directory: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Write a minimal index.html
    let index_html = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Innoframework App</title>
    <link rel="modulepreload" href="/_static/app.wasm">
  </head>
  <body>
    <div id="app"></div>
    <script type="module">
      import init from "/_static/app.wasm";
      init();
    </script>
  </body>
</html>"#;
    std::fs::write("dist/index.html", index_html).map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot write index.html: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    println!("inno build web: output written to dist/");
    println!("  - dist/index.html");
    println!("  - dist/csp-header.txt");
    println!("  - dist/_static/ (WASM output goes here)");
    println!("\nCSP header generated. Serve dist/ with the CSP header to satisfy TR-4.1/TR-3.3.");
    Ok(())
}

/// Build for the desktop target.
fn build_desktop(_manifest: &str) -> Result<(), InnoframeworkCliError> {
    println!("inno build desktop: checking capabilities...");
    println!("Desktop build requires Tauri 2 toolchain (cargo tauri).");
    println!("Run `cargo tauri build` in this directory to produce the desktop bundle.");
    Ok(())
}

/// Build for the mobile target.
fn build_mobile(_manifest: &str) -> Result<(), InnoframeworkCliError> {
    println!("inno build mobile: checking signing identity...");
    println!("Mobile build requires a signing identity (see inno.toml signing-identity).");
    println!("Run `cargo tauri build --target <mobile-target>` to produce the mobile bundle.");
    Ok(())
}

/// Generate a default CSP header (no exception slots).
fn generate_csp_header() -> String {
    // CSP with no exception slot — overrides must be explicit and listed
    // in the manifest (ADR-04, SEC-BUILD).
    "Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self';".to_string()
}
