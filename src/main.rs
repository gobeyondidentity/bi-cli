mod beyond_identity;
mod common;
mod okta;
mod onelogin;

use crate::common::command::ambassador_impl_Executable;
use async_trait::async_trait;
use beyond_identity::api::commands::BeyondIdentityApiCommands;
use beyond_identity::commands::BeyondIdentityHelperCommands;
use clap::{Args, Parser, Subcommand};
use clap_markdown::MarkdownOptions;
use common::command::Executable;
use common::error::BiError;
use log::LevelFilter;
use okta::commands::OktaCommands;
use onelogin::commands::OneloginCommands;

#[derive(Parser)]
#[clap(
    name = "bi",
    about = "Official Beyond Identity command-line interface.",
    version = env!("CARGO_PKG_VERSION"), // Dynamically pulls the version from Cargo.toml
    long_about = None
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[clap(short, long)]
    log_level: Option<String>,
}

#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
enum Commands {
    /// Commands related to Beyond Identity API
    #[clap(subcommand)]
    Api(BeyondIdentityApiCommands),

    /// Commands related to Beyond Identity API helper functions
    #[clap(subcommand)]
    Helper(BeyondIdentityHelperCommands),

    /// Commands related to Okta
    #[clap(subcommand)]
    Okta(OktaCommands),

    /// Commands related to OneLogin
    #[clap(subcommand)]
    Onelogin(OneloginCommands),

    /// Generate Markdown
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

    cli.command
        .execute()
        .await
        .expect("Failed to execute command");
}
