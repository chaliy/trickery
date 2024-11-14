use clap::{Args, ValueHint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::read_to_string;

use super::super::trickery::generate::generate_from_template;
use super::{CommandExec, CommandResult};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResult {
    output: String,
}

impl CommandResult<GenerateResult> for GenerateResult {
    fn get_result(&self) -> &GenerateResult {
        &self
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

        let template: String = read_to_string(input_path).await?;

        let output = generate_from_template(&template, input_variables).await?;

        match context.get_cli().is_interactive() {
            true => {
                println!("{}", output);
            }
            false => (),
        };

        return Ok(Box::from(GenerateResult { output }));
    }
}
