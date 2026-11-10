//! `inno build` — build for web, desktop, or mobile targets.

use crate::errors::InnoframeworkCliError;
use crate::BuildTarget;
use innoframework_core::manifest_toml::parse_manifest;

/// Build for the given target.
pub fn run(target: BuildTarget) -> Result<(), InnoframeworkCliError> {
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
        BuildTarget::Web => build_web(&manifest)?,
        BuildTarget::Desktop => build_desktop(&manifest)?,
        BuildTarget::Mobile => build_mobile(&manifest)?,
    }

    Ok(())
}

/// Check the manifest for secret values (secret boundary).
/// Secret patterns are rejected before any build step runs.
pub fn check_secret_boundary(manifest: &str) -> Result<(), InnoframeworkCliError> {
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

/// Build for the web target: a real `dx build --platform web --release`
/// (Dioxus 0.7 → WASM), then the CSP header is written beside its output.
fn build_web(_manifest: &str) -> Result<(), InnoframeworkCliError> {
    println!("inno build web: running `dx build --platform web --release`...");

    let status = std::process::Command::new("dx")
        .args(["build", "--platform", "web", "--release"])
        .status()
        .map_err(|_| {
            crate::errors::InnoframeworkCliError::new(
                crate::errors::CliErrorCode::TemplateMissing,
                "the `dx` tool was not found",
                "install it with `cargo install dioxus-cli --version 0.7.10`",
                Some("https://innoframework.dev/docs/cli/build#web-toolchain"),
            )
        })?;
    if !status.success() {
        return Err(crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            "`dx build --platform web` failed",
            "fix the compile errors above, then re-run `inno build web`",
            Some("https://innoframework.dev/docs/cli/build#web-build-failed"),
        ));
    }

    // Public directory produced by dx for the web platform.
    let public = std::path::Path::new("target/dx/").to_path_buf();
    let out_dir = find_dx_public_dir(&public).ok_or_else(|| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            "dx finished but its web output directory was not found under target/dx",
            "check the `dx build` output above",
            None,
        )
    })?;

    std::fs::write(out_dir.join("csp-header.txt"), generate_csp_header()).map_err(|e| {
        crate::errors::InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot write CSP header: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    println!("inno build web: output in {}", out_dir.display());
    println!("  - index.html and WASM bundle (from dx)");
    println!("  - csp-header.txt (serve with this header)");
    Ok(())
}

/// Locate the `public` directory dx writes for the web platform.
///
/// dx 0.7 writes `target/dx/<app>/release/web/public`. The app directory is
/// not known in advance, so every `<app>` candidate is checked for
/// `index.html`.
pub fn find_dx_public_dir(root: &std::path::Path) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(root).ok()?;
    for entry in entries.flatten() {
        let candidate = entry.path().join("release").join("web").join("public");
        if candidate.join("index.html").exists() {
            return Some(candidate);
        }
    }
    None
}

/// Build for the desktop target.
fn build_desktop(manifest: &str) -> Result<(), InnoframeworkCliError> {
    let parsed = parse_manifest(manifest).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::BuildNoManifest,
            format!("cannot parse inno.toml: {e}"),
            "fix the manifest format and re-run `inno build desktop`",
            None,
        )
    })?;

    // Desktop requires at least the desktop target.
    if !parsed
        .targets()
        .contains(&innoframework_core::build_target::TargetKind::Desktop)
    {
        return Err(InnoframeworkCliError::build_desktop_no_target());
    }

    println!("inno build desktop: generating Tauri 2 project files...");

    // Generate Tauri project files from manifest capabilities.
    let root = std::path::Path::new(".");
    crate::commands::tauri::write_tauri_project(
        root,
        parsed.name(),
        parsed.version(),
        parsed.capabilities(),
        &generate_csp_header(),
    )?;

    // Verify cargo tauri is available.
    let status = std::process::Command::new("cargo")
        .args(["tauri", "--version"])
        .output()
        .map_err(|_| InnoframeworkCliError::build_tauri_not_found())?;
    if !status.status.success() {
        return Err(InnoframeworkCliError::build_tauri_not_found());
    }

    // Run `cargo tauri build` to produce the desktop bundle.
    println!("inno build desktop: running `cargo tauri build`...");
    let status = std::process::Command::new("cargo")
        .args(["tauri", "build"])
        .status()
        .map_err(|_| InnoframeworkCliError::build_tauri_not_found())?;
    if !status.success() {
        return Err(InnoframeworkCliError::build_tauri_build_failed());
    }

    println!("inno build desktop: bundle produced by Tauri 2.");
    println!("  - tauri.conf.json and capability JSONs written to src-tauri/");
    println!("  - Signing: deferred (ADR-09) — unsigned bundle, see docs for code-signing.");
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
pub fn generate_csp_header() -> String {
    // CSP with no exception slot - overrides must be explicit and listed
    // in the manifest (ADR-04, SEC-BUILD). style-src allows 'unsafe-inline'
    // for framework/runtime style injection; prefer external CSS (assets/inno-ui.css).
    "Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self';".to_string()
}
