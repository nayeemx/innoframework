//! `inno init` wizard — scaffolds a new project through interactive prompts.
//!
//! The wizard asks for target platforms, data backend, AI provider, and auth
//! method. Each answer produces a concrete config diff shown before writing
//! (TR-2.2).

use crate::errors::{CliErrorCode, InnoframeworkCliError};
use std::fs;
use std::io::{self, Write};

/// Answers collected during the wizard.
#[derive(Debug, Clone)]
pub struct WizardAnswers {
    pub project_name: String,
    pub platforms: Vec<Platform>,
    pub data_backend: DataBackend,
    pub ai_provider: AiProvider,
    pub auth_method: AuthMethod,
}

/// Target platforms for the generated project.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Web,
    Desktop,
    Mobile,
}

/// Data backend for the project.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataBackend {
    FirebaseDev,
    Postgres,
    Sqlite,
}

/// AI provider for the project.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiProvider {
    None,
    LlamaCppLocal,
    GeminiRemote,
}

/// Auth method for the project.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthMethod {
    Session,
    None,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Web => "web",
            Platform::Desktop => "desktop",
            Platform::Mobile => "mobile",
        }
    }

    pub fn all() -> Vec<Platform> {
        vec![Platform::Web, Platform::Desktop, Platform::Mobile]
    }
}

impl DataBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataBackend::FirebaseDev => "firebase",
            DataBackend::Postgres => "postgres",
            DataBackend::Sqlite => "sqlite",
        }
    }

    pub fn all() -> Vec<DataBackend> {
        vec![
            DataBackend::FirebaseDev,
            DataBackend::Postgres,
            DataBackend::Sqlite,
        ]
    }
}

impl AiProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::None => "none",
            AiProvider::LlamaCppLocal => "llama.cpp",
            AiProvider::GeminiRemote => "gemini",
        }
    }

    pub fn all() -> Vec<AiProvider> {
        vec![
            AiProvider::None,
            AiProvider::LlamaCppLocal,
            AiProvider::GeminiRemote,
        ]
    }
}

impl AuthMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthMethod::Session => "session",
            AuthMethod::None => "none",
        }
    }

    pub fn all() -> Vec<AuthMethod> {
        vec![AuthMethod::Session, AuthMethod::None]
    }
}

/// Run the init wizard.
pub fn run(name: Option<String>, non_interactive: bool) -> Result<(), InnoframeworkCliError> {
    let project_name = name.unwrap_or_else(current_dir_name);

    let answers = if non_interactive {
        WizardAnswers {
            project_name: project_name.clone(),
            platforms: vec![Platform::Web],
            data_backend: DataBackend::FirebaseDev,
            ai_provider: AiProvider::None,
            auth_method: AuthMethod::Session,
        }
    } else {
        run_interactive(&project_name)?
    };

    // Show the config diff before writing (TR-2.2).
    show_config_diff(&answers)?;

    // Confirm before writing. Non-interactive mode (-y) skips the prompt.
    if !non_interactive {
        print!("Write project files? [y/N]: ");
        io::stdout().flush().map_err(io_error)?;
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).map_err(io_error)?;
        if !confirm.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    write_project_files(&answers)?;
    println!("Project '{}' created.", answers.project_name);
    println!(
        "Next: `cd {}` then `inno dev` or `inno build web`.",
        answers.project_name
    );
    Ok(())
}

fn current_dir_name() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "my-app".to_string())
}

fn run_interactive(project_name: &str) -> Result<WizardAnswers, InnoframeworkCliError> {
    let platforms = select_multi("Target platforms", Platform::all(), |p| p.as_str())?;
    let data_backend = select_single("Data backend", DataBackend::all(), |b| b.as_str())?;
    let ai_provider = select_single("AI provider", AiProvider::all(), |p| p.as_str())?;
    let auth_method = select_single("Auth method", AuthMethod::all(), |m| m.as_str())?;

    Ok(WizardAnswers {
        project_name: project_name.to_string(),
        platforms,
        data_backend,
        ai_provider,
        auth_method,
    })
}

fn select_multi<T: Clone>(
    prompt: &str,
    options: Vec<T>,
    label: impl Fn(&T) -> &'static str,
) -> Result<Vec<T>, InnoframeworkCliError> {
    println!("\n{prompt}");
    for (i, opt) in options.iter().enumerate() {
        println!("  {}) {}", i + 1, label(opt));
    }
    println!("  Enter numbers separated by spaces or commas (e.g. 1,3):");
    print!("> ");
    io::stdout().flush().map_err(io_error)?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(io_error)?;

    let mut selected = Vec::new();
    for token in input.split(|c: char| c.is_whitespace() || c == ',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let idx: usize = token.parse().map_err(|_| {
            InnoframeworkCliError::new(
                CliErrorCode::InitInvalidPlatform,
                format!("invalid selection '{token}'"),
                "enter numbers from the list above",
                None,
            )
        })?;
        if idx == 0 || idx > options.len() {
            return Err(InnoframeworkCliError::new(
                CliErrorCode::InitInvalidPlatform,
                format!("selection {idx} out of range"),
                "enter numbers from the list above",
                None,
            ));
        }
        selected.push(options[idx - 1].clone());
    }

    if selected.is_empty() {
        return Err(InnoframeworkCliError::new(
            CliErrorCode::InitInvalidPlatform,
            "no platforms selected",
            "select at least one platform",
            None,
        ));
    }

    Ok(selected)
}

fn select_single<T: Clone + Copy>(
    prompt: &str,
    options: Vec<T>,
    label: impl Fn(&T) -> &'static str,
) -> Result<T, InnoframeworkCliError> {
    println!("\n{prompt}");
    for (i, opt) in options.iter().enumerate() {
        println!("  {}) {}", i + 1, label(opt));
    }
    print!("> ");
    io::stdout().flush().map_err(io_error)?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(io_error)?;
    let idx: usize = input.trim().parse().map_err(|_| {
        InnoframeworkCliError::new(
            CliErrorCode::InitInvalidPlatform,
            "invalid selection",
            "enter a number from the list above",
            None,
        )
    })?;
    if idx == 0 || idx > options.len() {
        return Err(InnoframeworkCliError::new(
            CliErrorCode::InitInvalidPlatform,
            format!("selection {idx} out of range"),
            "enter a number from the list above",
            None,
        ));
    }
    Ok(options[idx - 1])
}

fn show_config_diff(answers: &WizardAnswers) -> Result<(), InnoframeworkCliError> {
    println!("\n=== Configuration diff (will be written) ===");
    println!("project: {}", answers.project_name);
    print!("targets: ");
    for (i, p) in answers.platforms.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", p.as_str());
    }
    println!();
    println!("data_backend: {}", answers.data_backend.as_str());
    println!("ai_provider: {}", answers.ai_provider.as_str());
    println!("auth: {}", answers.auth_method.as_str());
    println!("========================================\n");
    Ok(())
}

fn write_project_files(answers: &WizardAnswers) -> Result<(), InnoframeworkCliError> {
    let dir = &answers.project_name;

    if std::path::Path::new(dir).exists() {
        return Err(InnoframeworkCliError::init_dir_exists(dir));
    }

    fs::create_dir_all(dir).map_err(|e| {
        InnoframeworkCliError::new(
            CliErrorCode::InitDirExists,
            format!("cannot create directory '{dir}': {e}"),
            "choose a different location or check permissions",
            None,
        )
    })?;

    // Write inno.toml manifest
    let manifest = generate_manifest(answers);
    fs::write(format!("{dir}/inno.toml"), manifest).map_err(|e| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            format!("cannot write inno.toml: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Write Cargo.toml
    let cargo_toml = generate_cargo_toml(answers);
    fs::write(format!("{dir}/Cargo.toml"), cargo_toml).map_err(|e| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            format!("cannot write Cargo.toml: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Write main.rs (web entry point)
    let src_dir = format!("{dir}/src");
    fs::create_dir_all(&src_dir).map_err(|e| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            format!("cannot create src directory: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    let main_rs = generate_main_rs(answers);
    fs::write(format!("{src_dir}/main.rs"), main_rs).map_err(|e| {
        InnoframeworkCliError::new(
            CliErrorCode::TemplateMissing,
            format!("cannot write src/main.rs: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    Ok(())
}

fn generate_manifest(answers: &WizardAnswers) -> String {
    let targets_lines: Vec<String> = answers
        .platforms
        .iter()
        .map(|p| format!("  \"{}\"", p.as_str()))
        .collect();
    let targets_joined = targets_lines.join(",\n");

    let capabilities = if answers.platforms.contains(&Platform::Desktop) {
        "\n[[capabilities]]\nid = \"fs.read\"\nplatforms = [\"desktop\"]\npath_prefixes = [\"data/\"]"
    } else {
        ""
    };

    format!(
        r#"[project]
name = "{}"
version = "0.1.0"
targets = [
{targets_joined}
]

[data]
backend = "{}"

[ai]
provider = "{}"

[auth]
method = "{}"
{capabilities}
"#,
        answers.project_name,
        answers.data_backend.as_str(),
        answers.ai_provider.as_str(),
        answers.auth_method.as_str(),
    )
}

fn generate_cargo_toml(answers: &WizardAnswers) -> String {
    let has_web = answers.platforms.contains(&Platform::Web);
    let web_deps = if has_web {
        "dioxus = { version = \"0.7\", features = [\"web\"] }\n"
    } else {
        ""
    };

    let content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
{web_deps}[[bin]]
name = "{}"
path = "src/main.rs"
"#,
        answers.project_name, answers.project_name
    );
    content
}

fn generate_main_rs(answers: &WizardAnswers) -> String {
    let has_web = answers.platforms.contains(&Platform::Web);
    if has_web {
        r#"use dioxus::prelude::*;

fn app() -> Element {
    rsx! {
        h1 { "Welcome to Innoframework" }
        p { "Your web app is running." }
    }
}

fn main() {
    LaunchBuilder::web().launch(app);
}
"#
        .to_string()
    } else {
        "fn main() {\n    println!(\"Innoframework project running.\");\n}\n".to_string()
    }
}

fn io_error(e: io::Error) -> InnoframeworkCliError {
    InnoframeworkCliError::new(
        CliErrorCode::InitDirExists,
        format!("I/O error: {e}"),
        "check permissions and try again",
        None,
    )
}
