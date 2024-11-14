use langchain_rust::{
    language_models::llm::LLM,
    llm::openai::OpenAI,
    prompt::{PromptFromatter, PromptTemplate, TemplateFormat},
};
use serde_json::Value;
use std::collections::HashMap;

pub async fn generate_from_template(
    template: &str,
    input_variables: HashMap<String, Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt_template = PromptTemplate::new(
        template.to_string(),
        input_variables.keys().cloned().collect(),
        TemplateFormat::Jinja2,
    );
    let prompt = prompt_template.format(input_variables)?;

    let response = generate(&prompt).await?;
    Ok(response)
}

pub async fn generate(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let open_ai = OpenAI::default();
    let response = open_ai.invoke(prompt).await?;
    Ok(response)
}
