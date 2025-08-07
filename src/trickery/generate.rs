use llm_chain::{executor, parameters, prompt, Parameters};
use serde_json::Value;
use std::collections::HashMap;

pub async fn generate_from_template(
    template: &str,
    input_variables: &HashMap<String, Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = prompt!(template);
    let vars = input_variables
        .iter()
        .fold(parameters!(), |acc, (k, v)| {
            acc.with(k, v.as_str().unwrap_or_default())
        });
    let exec = executor!()?;
    let res = exec.run(prompt, vars).await?;
    let output = res.to_string();
    Ok(output)
}

pub async fn generate(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let exec = executor!()?;
    let res = exec.run(prompt!(prompt), parameters!()).await?;
    let output = res.to_string();
    Ok(output)
}


