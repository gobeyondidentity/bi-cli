use std::env;

use crate::beyond_identity::api::common::api_client::ApiClient;
use crate::common::command::ambassador_impl_Executable;
use crate::common::database::models::AiProvider;
use crate::common::{command::Executable, error::BiError};
use crate::Cli;

use async_trait::async_trait;
use clap::{Args, Subcommand};
use clap_markdown::MarkdownOptions;
use reqwest::Client;
use serde_json::json;

// ====================================
// Ai Commands
// ====================================

/// Commands for interacting with the AI helper tool to assist with CLI operations.
#[derive(Subcommand, ambassador::Delegate)]
#[delegate(Executable)]
pub enum AiCommands {
    /// Ask the AI helper tool for assistance in generating CLI commands.
    Ask(Ask),
}

// ====================================
// Ai Ask
// ====================================

#[derive(Args)]
pub struct Ask {
    /// The question or command you need assistance with.
    pub input: String,
}

#[async_trait]
impl Executable for Ask {
    async fn execute(&self) -> Result<(), BiError> {
        let prompt = prompt(&self.input);

        let api_client = ApiClient::new(None, None).await;

        let default_ai_provider = match api_client.db.get_default_ai_provider().await? {
            Some(x) => x,
            None => {
                return Err(BiError::StringError(
                    "No default AI provider set".to_string(),
                ));
            }
        };

        Ok(println!(
            "{}",
            match default_ai_provider {
                AiProvider::Openai => openai(&api_client, &prompt).await?,
                AiProvider::Anthropic => anthropic(&api_client, &prompt).await?,
            }
        ))
    }
}

fn prompt(input: &str) -> String {
    // Extract OS and terminal information
    let os = env::consts::OS;
    let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let term = env::var("TERM").unwrap_or_else(|_| "unknown".to_string());
    let invocation_name = env::args().next().unwrap_or_else(|| "bi".to_string());

    let tool_summary = clap_markdown::help_markdown_custom::<Cli>(
        &MarkdownOptions::new()
            .title(invocation_name.clone())
            .show_footer(false)
            .show_table_of_contents(true),
    );

    log::debug!(
        "Inputs - OS: {}, Shell: {}, Terminal: {}, Program Invocation: {}, User Input: {}",
        os,
        shell,
        term,
        invocation_name,
        input.trim(),
    );

    format!(
            "You are an assistant that generates shell commands using the 'bi' CLI tool based on the user's request.

    User's Environment:
    - OS: {}
    - Shell: {}
    - Terminal: {}
    - Program Invocation: {}

    'bi' CLI Tool Summary:
    {}

    User's request:
    {}

    Provide the shell command only as a one liner, using '{}' as the program invocation, without any additional explanation. Just text, no code snippet back ticks.",
            os,
            shell,
            term,
            invocation_name,
            tool_summary,
            input.trim(),
            invocation_name,
        )
}

async fn openai(api_client: &ApiClient, prompt: &str) -> Result<String, BiError> {
    let api_key = match api_client.db.get_openai_config().await? {
        Some(x) => x.api_key,
        None => {
            return Err(BiError::StringError(
                "No api_key set for Openai".to_string(),
            ));
        }
    };

    let response = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return response
            .text()
            .await
            .map_err(|e| BiError::StringError(e.to_string()));
    }

    let res_json: serde_json::Value = response.json().await?;

    // Extract the generated command
    let generated_command = res_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response");

    Ok(generated_command.to_string())
}

pub async fn anthropic(api_client: &ApiClient, prompt: &str) -> Result<String, BiError> {
    let api_key = match api_client.db.get_anthropic_config().await? {
        Some(x) => x.api_key,
        None => {
            return Err(BiError::StringError(
                "No api_key set for Anthropic".to_string(),
            ));
        }
    };

    let response = Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1024,
            "messages": [{"role": "user", "content": prompt}],
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return response
            .text()
            .await
            .map_err(|e| BiError::StringError(e.to_string()));
    }

    let res_json: serde_json::Value = response.json().await?;

    let generated_response = res_json["content"][0]["text"]
        .as_str()
        .unwrap_or("No response");

    Ok(generated_response.to_string())
}
