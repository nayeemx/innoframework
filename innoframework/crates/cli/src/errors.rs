//! Actionable error catalog for the CLI (TR-2.4).
//!
//! Every error code maps to a fix sentence and a doc link. Errors are
//! fail-closed: the CLI prints the code, the user-facing message, and the
//! fix, then exits non-zero.

use std::fmt;

/// Stable CLI error codes. Format: AUD-3xxx.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CliErrorCode {
    /// Target directory already exists.
    InitDirExists = 3001,
    /// Invalid platform selection in the wizard.
    InitInvalidPlatform = 3002,
    /// No inno.toml manifest found in current directory.
    BuildNoManifest = 3003,
    /// Secret detected in manifest value.
    BuildSecretInManifest = 3004,
    /// Dev server port already in use.
    DevPortInUse = 3005,
    /// Unknown build target.
    BuildUnknownTarget = 3006,
    /// Template file missing from the templates directory.
    TemplateMissing = 3007,
    /// The Rust toolchain is older than the pinned minimum.
    ToolchainTooOld = 3008,
}

/// An actionable error: code, user message, fix sentence, doc link.
#[derive(Debug)]
pub struct InnoframeworkCliError {
    pub code: CliErrorCode,
    pub message: String,
    pub fix: String,
    pub doc_link: Option<String>,
}

impl InnoframeworkCliError {
    pub fn new(
        code: CliErrorCode,
        message: impl Into<String>,
        fix: impl Into<String>,
        doc_link: Option<&str>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            fix: fix.into(),
            doc_link: doc_link.map(|s| s.to_string()),
        }
    }

    /// Print the error in a user-friendly format.
    pub fn print(&self) {
        let code_str = match self.code {
            CliErrorCode::InitDirExists => "AUD-3001",
            CliErrorCode::InitInvalidPlatform => "AUD-3002",
            CliErrorCode::BuildNoManifest => "AUD-3003",
            CliErrorCode::BuildSecretInManifest => "AUD-3004",
            CliErrorCode::DevPortInUse => "AUD-3005",
            CliErrorCode::BuildUnknownTarget => "AUD-3006",
            CliErrorCode::TemplateMissing => "AUD-3007",
            CliErrorCode::ToolchainTooOld => "AUD-3008",
        };
        eprintln!("{code_str}: {}", self.message);
        eprintln!("  Fix: {}", self.fix);
        if let Some(link) = &self.doc_link {
            eprintln!("  Docs: {link}");
        }
    }
}

impl fmt::Display for InnoframeworkCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.fix)
    }
}

impl std::error::Error for InnoframeworkCliError {}

/// Convenience constructors for common errors.
#[allow(dead_code)]
impl InnoframeworkCliError {
    pub fn init_dir_exists(path: &str) -> Self {
        Self::new(
            CliErrorCode::InitDirExists,
            format!("directory '{path}' already exists"),
            "choose a different project name or remove the existing directory",
            Some("https://innoframework.dev/docs/cli/init#dir-exists"),
        )
    }

    pub fn init_invalid_platform(platform: &str) -> Self {
        Self::new(
            CliErrorCode::InitInvalidPlatform,
            format!("'{platform}' is not a valid platform"),
            "choose one of: web, desktop, mobile",
            Some("https://innoframework.dev/docs/cli/init#platforms"),
        )
    }

    pub fn build_no_manifest() -> Self {
        Self::new(
            CliErrorCode::BuildNoManifest,
            "no inno.toml manifest found in the current directory",
            "run `inno init` first, or change into a project directory",
            Some("https://innoframework.dev/docs/cli/build#no-manifest"),
        )
    }

    pub fn build_secret_in_manifest(field: &str) -> Self {
        Self::new(
            CliErrorCode::BuildSecretInManifest,
            format!("manifest field '{field}' contains a secret value"),
            "remove the secret from inno.toml and set it via `inno config set`",
            Some("https://innoframework.dev/docs/cli/build#secret-in-manifest"),
        )
    }

    pub fn dev_port_in_use(port: u16) -> Self {
        Self::new(
            CliErrorCode::DevPortInUse,
            format!("port {port} is already in use"),
            "choose a different port with `--port` or stop the process using {port}",
            Some("https://innoframework.dev/docs/cli/dev#port-in-use"),
        )
    }

    pub fn build_unknown_target(target: &str) -> Self {
        Self::new(
            CliErrorCode::BuildUnknownTarget,
            format!("'{target}' is not a valid build target"),
            "choose one of: web, desktop, mobile",
            Some("https://innoframework.dev/docs/cli/build#targets"),
        )
    }

    pub fn template_missing(template: &str) -> Self {
        Self::new(
            CliErrorCode::TemplateMissing,
            format!("template '{template}' not found"),
            "reinstall the framework or report the missing template",
            Some("https://innoframework.dev/docs/cli/init#template-missing"),
        )
    }

    pub fn toolchain_too_old(found: &str, required: &str) -> Self {
        Self::new(
            CliErrorCode::ToolchainTooOld,
            format!("Rust toolchain {found} is older than the required {required}"),
            "install a newer toolchain with `rustup update`",
            Some("https://innoframework.dev/docs/cli#toolchain"),
        )
    }
}
