use clap::{Args, ValueHint};
use llm_chain::agents::self_ask_with_search::Agent;
use llm_chain::agents::self_ask_with_search::EarlyStoppingConfig;
use llm_chain::executor;
use llm_chain::tools::tools::BashTool;
use llm_chain::Parameters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::read_to_string;

use super::{CommandExec, CommandResult};

use crate::trickery::generate::generate_from_template;

fn parse_key_val(s: &str) -> Result<(String, Value), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=VALUE: no `=` found in `{}`", s))?;
    Ok((
        s[..pos].to_string(),
        Value::String(s[pos + 1..].to_string()),
    ))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentResult {
    output: String,
}

impl CommandResult<AgentResult> for AgentResult {
    fn get_result(&self) -> &AgentResult {
        self
    }
}

#[derive(Args)]
pub struct AgentArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub input: Option<PathBuf>,
    /// Variables to be used in prompt
    #[arg(short, long="var", value_parser = parse_key_val, number_of_values = 1)]
    pub vars: Vec<(String, Value)>,
}

impl CommandExec<AgentResult> for AgentArgs {
    async fn exec(
        &self,
        context: &impl super::CommandExecutionContext,
    ) -> Result<Box<dyn CommandResult<AgentResult>>, Box<dyn std::error::Error>> {
        let input_path = self
            .input
            .as_ref()
            .ok_or("Input file path is required")?;
        let template = read_to_string(input_path).await?;
        let input_variables: HashMap<String, Value> = self.vars.clone().into_iter().collect();

        let prompt = generate_from_template(&template, &input_variables).await?;

        let exec = executor!()?;
        let agent = Agent::new(
            exec,
            BashTool::new(),
            EarlyStoppingConfig::new_leaping_rabbit(),
        );

        let (res, intermediate_steps) = agent.run(&prompt).await?;

        if context.get_cli().is_interactive() {
            for step in intermediate_steps {
                println!("Thought: {}", step.thought);
                println!("Action: {}", step.action.tool_name());
                println!("Action Input: {}", step.action.tool_input());
                println!("Observation: {}", step.observation.output());
            }
            println!("Final Answer: {}", res.return_value);
        }

        let output = res.return_value;
        Ok(Box::from(AgentResult { output }))
    }
}
