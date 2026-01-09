use crate::provider::openai::OpenAIProvider;
use crate::provider::{substitute_variables, CompletionRequest, Message, ReasoningLevel, Tool};
use serde_json::Value;
use std::collections::HashMap;

/// Configuration for template generation
#[derive(Debug, Clone, Default)]
pub struct GenerateConfig {
    pub model: Option<String>,
    pub reasoning_level: Option<ReasoningLevel>,
    pub tools: Option<Vec<Tool>>,
    pub max_tokens: Option<u32>,
}

/// Generate text from template with variable substitution.
/// Uses OpenAI provider by default.
pub async fn generate_from_template(
    template: &str,
    input_variables: &HashMap<String, Value>,
    config: GenerateConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    // Substitute template variables
    let prompt_text = substitute_variables(template, input_variables);

    // Create provider and request
    let provider = OpenAIProvider::from_env()?;

    let mut request = CompletionRequest::new(vec![Message::user(prompt_text)]);

    if let Some(model) = config.model {
        request = request.with_model(model);
    }
    if let Some(level) = config.reasoning_level {
        request = request.with_reasoning_level(level);
    }
    if let Some(tools) = config.tools {
        request = request.with_tools(tools);
    }
    if let Some(max_tokens) = config.max_tokens {
        request = request.with_max_tokens(max_tokens);
    }

    let response = provider.complete(request).await?;

    // If we have tool calls, return them as JSON for processing
    if let Some(tool_calls) = response.tool_calls {
        return Ok(serde_json::to_string_pretty(&tool_calls)?);
    }

    Ok(response.content.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_default() {
        let config = GenerateConfig::default();
        assert!(config.model.is_none());
        assert!(config.reasoning_level.is_none());
        assert!(config.tools.is_none());
    }

    #[test]
    fn test_generate_config_with_values() {
        let config = GenerateConfig {
            model: Some("gpt-4o".to_string()),
            reasoning_level: Some(ReasoningLevel::High),
            tools: None,
            max_tokens: Some(1000),
        };
        assert_eq!(config.model, Some("gpt-4o".to_string()));
        assert_eq!(config.reasoning_level, Some(ReasoningLevel::High));
    }
}
