use langchain_rust::{language_models::llm::LLM, llm::openai::OpenAI};

pub async fn generate(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let open_ai = OpenAI::default();
    let response = open_ai.invoke(prompt).await?;
    Ok(response)
}