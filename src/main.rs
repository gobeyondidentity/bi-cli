mod ai;
mod beyond_identity;
mod common;
mod config;
mod okta;
mod onelogin;

use ai::command::AiCommands;
use async_trait::async_trait;
use beyond_identity::api::common::command::BeyondIdentityApiCommands;
use beyond_identity::helper::command::BeyondIdentityHelperCommands;
use clap::{Args, Parser, Subcommand};
use clap_markdown::MarkdownOptions;
use common::command::{ambassador_impl_Executable, Executable};
use common::error::BiError;
use config::command::ConfigCommands;
use log::LevelFilter;
use okta::command::OktaCommands;
use onelogin::command::OneloginCommands;

#[derive(Parser)]
#[clap(
    name = "bi",
    about = "Official Beyond Identity command-line interface.",
    version = env!("CARGO_PKG_VERSION"), // Dynamically pulls the version from Cargo.toml
    long_about = None
)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    log_level: Option<String>,
}

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
enum Commands {
    /// Manage CLI tool configuration settings
    #[clap(subcommand)]
    Config(ConfigCommands),

    /// Interact with Beyond Identity API endpoints
    #[clap(subcommand)]
    Api(BeyondIdentityApiCommands),

    /// Access helper functions for Beyond Identity API operations
    #[clap(subcommand)]
    Helper(BeyondIdentityHelperCommands),

    /// Helper tool to generate example commands for CLI operations
    #[clap(subcommand)]
    Ai(AiCommands),

    /// Commands solely for fast migration off of Okta
    #[clap(subcommand)]
    Okta(OktaCommands),

    /// Commands solely for fast migration off of OneLogin
    #[clap(subcommand)]
    Onelogin(OneloginCommands),

    /// Generate Markdown documentation (hidden)
    #[clap(hide = true)]
    GenerateMarkdown(GenerateMarkdownCommand),
}

#[derive(Clone, Debug, Args)]
struct GenerateMarkdownCommand;

#[async_trait]
impl Executable for GenerateMarkdownCommand {
    async fn execute(&self) -> Result<(), BiError> {
        println!(
            "{}",
            clap_markdown::help_markdown_custom::<Cli>(
                &MarkdownOptions::new()
                    .title(format!("bi"))
                    .show_footer(false)
                    .show_table_of_contents(true),
            )
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let log_level = match cli.log_level.as_deref() {
        // Use for logging error events that indicate a failure in the application.
        Some("error") => LevelFilter::Error,
        // Use for logging potentially harmful situations that might need attention.
        Some("warn") => LevelFilter::Warn,
        // Use for logging informational messages that highlight the progress of the application.
        Some("info") => LevelFilter::Info,
        // Use for logging detailed information useful for debugging.
        Some("debug") => LevelFilter::Debug,
        // Use for logging very detailed and fine-grained information, typically for tracing program execution.
        Some("trace") => LevelFilter::Trace,
        // Logging is defaulted to info if none is specified.
        _ => LevelFilter::Info,
    };
    env_logger::Builder::new().filter(None, log_level).init();

    match cli.command.execute().await {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e.to_string()),
    }
}
