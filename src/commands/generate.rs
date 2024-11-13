use std::path::PathBuf;
use clap::{Args, ValueHint};
use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;

use super::{CommandResult, CommandExec};
use super::super::trickery::generate::generate;


#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResult {
    output: String
}

impl CommandResult<GenerateResult> for GenerateResult {
    fn get_result(&self) -> &GenerateResult {
        &self
    }
}

#[derive(Args)]
pub struct GenerateArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,
}


impl CommandExec<GenerateResult> for GenerateArgs {
    async fn exec(&self, context: &impl super::CommandExecutionContext) -> Result<Box<dyn CommandResult<GenerateResult>>, Box<dyn std::error::Error>> {
        
        let input_path = match &self.input {
            Some(path) => path,
            None => return Err("Input file path is required".into()),
        };

        let prompt = read_to_string(input_path).await?;

        let output = generate(&prompt).await?;

        match context.get_cli().is_interactive() {
            true => {
                println!("{}", output);
            },
            false => ()
        };

        return Ok(Box::from(GenerateResult { output }));
    }
}