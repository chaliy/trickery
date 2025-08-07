use llm_chain::prompt::chat::{ChatPrompt, ChatPromptBuilder};
use llm_chain::prompt::Prompt;
use llm_chain::tools::Tool;
use llm_chain::{executor, parameters, prompt};
use serde_json::Value;
use std::collections::HashMap;

pub async fn generate_from_template(
    template: &str,
    input_variables: &HashMap<String, Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut p = prompt!(template);
    for (k, v) in input_variables {
        p = p.with_parameter(k, v.as_str().unwrap_or(""));
    }
    generate(p).await
}

pub async fn generate<P: Prompt + Send + Sync>(
    prompt: P,
) -> Result<String, Box<dyn std::error::Error>> {
    let exec = executor!()?;
    let res = exec.run(prompt, parameters!()).await?;
    let output = res.to_string();
    Ok(output)
}

