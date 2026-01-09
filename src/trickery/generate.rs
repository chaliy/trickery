use llm_chain::options::ModelRef;
use llm_chain::step::Step;
use llm_chain::{executor, options, parameters, prompt};
use serde_json::Value;
use std::collections::HashMap;

pub async fn generate_from_template(
    template: &str,
    input_variables: &HashMap<String, Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = prompt!(template);
    let vars = input_variables.iter().fold(parameters!(), |acc, (k, v)| {
        acc.with(k, v.as_str().unwrap_or_default())
    });
    let exec = executor!(
        chatgpt,
        options!(
            Model: ModelRef::from_model_name("gpt-5-mini")
        )
    )?;
    let res = Step::for_prompt_template(prompt).run(&vars, &exec).await?;
    let output = res.to_string();
    Ok(output)
}
