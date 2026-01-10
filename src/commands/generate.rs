use clap::{Args, ValueHint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::read_to_string;

use super::super::trickery::generate::{generate_from_template, GenerateConfig};
use super::{CommandExec, CommandResult};
use crate::provider::ReasoningLevel;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResult {
    output: String,
}

impl CommandResult<GenerateResult> for GenerateResult {
    fn get_result(&self) -> &GenerateResult {
        self
    }
}

fn parse_key_val(s: &str) -> Result<(String, Value), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=VALUE: no `=` found in `{}`", s))?;
    Ok((
        s[..pos].to_string(),
        Value::String(s[pos + 1..].to_string()),
    ))
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,

    /// Variables to be used in prompt
    #[arg(short, long="var", value_parser = parse_key_val, number_of_values = 1)]
    vars: Vec<(String, Value)>,

    /// Model to use (e.g., gpt-5.2, gpt-5-mini, o1, o3-mini)
    #[arg(short, long)]
    model: Option<String>,

    /// Reasoning level for o1/o3 models: low, medium, high
    #[arg(short, long, value_parser = parse_reasoning_level)]
    reasoning: Option<ReasoningLevel>,

    /// Maximum tokens in response
    #[arg(long)]
    max_tokens: Option<u32>,

    /// Image files or URLs to include in the prompt (can be specified multiple times)
    #[arg(long)]
    image: Vec<String>,

    /// Image detail level: auto, low, high (default: auto)
    #[arg(long, default_value = "auto")]
    image_detail: String,
}

fn parse_reasoning_level(s: &str) -> Result<ReasoningLevel, String> {
    s.parse()
}

impl CommandExec<GenerateResult> for GenerateArgs {
    async fn exec(
        &self,
        context: &impl super::CommandExecutionContext,
    ) -> Result<Box<dyn CommandResult<GenerateResult>>, Box<dyn std::error::Error>> {
        let input_path = match &self.input {
            Some(path) => path,
            None => return Err("Input file path is required".into()),
        };

        let input_variables: HashMap<String, Value> = self
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let template: String = read_to_string(input_path).await.map_err(|e| {
            format!(
                "Failed to read input file '{}': {}",
                input_path.display(),
                e
            )
        })?;

        let images: Vec<String> = self.image.clone();

        let config = GenerateConfig {
            model: self.model.clone(),
            reasoning_level: self.reasoning,
            tools: None,
            max_tokens: self.max_tokens,
            images: if images.is_empty() {
                None
            } else {
                Some(images)
            },
            image_detail: Some(self.image_detail.clone()),
        };

        let output = generate_from_template(&template, &input_variables, config).await?;

        if context.get_cli().is_interactive() {
            println!("{}", output);
        };

        Ok(Box::from(GenerateResult { output }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_val() {
        let (key, val) = parse_key_val("name=John").unwrap();
        assert_eq!(key, "name");
        assert_eq!(val, Value::String("John".to_string()));
    }

    #[test]
    fn test_parse_key_val_with_equals_in_value() {
        let (key, val) = parse_key_val("expr=a=b").unwrap();
        assert_eq!(key, "expr");
        assert_eq!(val, Value::String("a=b".to_string()));
    }

    #[test]
    fn test_parse_key_val_error() {
        let result = parse_key_val("no_equals");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_reasoning_level() {
        assert_eq!(parse_reasoning_level("low").unwrap(), ReasoningLevel::Low);
        assert_eq!(
            parse_reasoning_level("medium").unwrap(),
            ReasoningLevel::Medium
        );
        assert_eq!(parse_reasoning_level("high").unwrap(), ReasoningLevel::High);
        assert!(parse_reasoning_level("invalid").is_err());
    }
}
