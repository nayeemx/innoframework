//! innoframework CLI — `inno` binary (L0 toolchain).
//!
//! Commands:
//! - `inno init` — scaffold a new project through an interactive wizard
//! - `inno dev` — run the dev server for a local project
//! - `inno build` — build for web/desktop/mobile targets

use clap::Parser;
use innoframework_cli::{commands, BuildTarget};

/// Innoframework CLI — scaffold, develop, and build Innoframework apps.
#[derive(Parser, Debug)]
#[command(name = "inno", version, about = "Innoframework CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Scaffold a new project through the init wizard.
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,
        /// Skip interactive prompts and use defaults
        #[arg(short = 'y', long = "yes")]
        non_interactive: bool,
    },
    /// Run the dev server for the current project.
    Dev {
        /// Port to listen on (default: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Generate documentation from source code.
    Docs {
        /// Source directory to scan (default: current directory)
        #[arg(short, long, default_value = ".")]
        source: String,
        /// Output file (default: print to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Build for a target: web, desktop, or mobile.
    Build {
        /// Target to build: web, desktop, mobile
        #[arg(value_enum)]
        target: BuildTarget,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init {
            name,
            non_interactive,
        } => commands::init::run(name, non_interactive),
        Commands::Dev { port } => commands::dev::run(port),
        Commands::Docs { source, output } => commands::docs::run(Some(source), output),
        Commands::Build { target } => commands::build::run(target),
    };

    if let Err(err) = result {
        err.print();
        std::process::exit(1);
    }
}
