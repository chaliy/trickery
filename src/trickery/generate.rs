use crate::provider::openai::OpenAIProvider;
use crate::provider::{CompletionRequest, ContentPart, ImageUrl, Message, ReasoningLevel, Tool};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

/// Configuration for template generation
#[derive(Debug, Clone, Default)]
pub struct GenerateConfig {
    pub model: Option<String>,
    pub reasoning_level: Option<ReasoningLevel>,
    pub tools: Option<Vec<Tool>>,
    pub max_tokens: Option<u32>,
    /// Image paths or URLs to include in the prompt
    pub images: Option<Vec<String>>,
    /// Image detail level: auto, low, high
    pub image_detail: Option<String>,
}

/// Convert an image path or URL to a format suitable for the API.
/// Local files are converted to base64 data URLs.
/// URLs starting with http:// or https:// are passed through unchanged.
fn image_to_url(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // If it's already a URL, return as-is
    if image_path.starts_with("http://") || image_path.starts_with("https://") {
        return Ok(image_path.to_string());
    }

    // It's a local file path - read and encode as base64
    let path = Path::new(image_path);
    let data = std::fs::read(path)?;

    // Detect MIME type from extension
    let mime_type = match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "image/png", // Default to PNG
    };

    let encoded = BASE64.encode(&data);
    Ok(format!("data:{};base64,{}", mime_type, encoded))
}

/// Substitute Jinja2-style template variables {{ var }} with values.
/// This is done BEFORE sending to the LLM provider.
pub fn substitute_variables(template: &str, variables: &HashMap<String, Value>) -> String {
    let mut result = template.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{ {} }}}}", key);
        let replacement = match value {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        result = result.replace(&placeholder, &replacement);
    }
    result
}

/// Generate text from template with variable substitution.
/// Uses OpenAI provider by default.
pub async fn generate_from_template(
    template: &str,
    input_variables: &HashMap<String, Value>,
    config: GenerateConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    // Substitute template variables BEFORE sending to provider
    let prompt_text = substitute_variables(template, input_variables);

    // Create provider and request
    let provider = OpenAIProvider::from_env()?;

    // Build message - use multimodal if images provided
    let message = if let Some(ref images) = config.images {
        let detail = config.image_detail.clone();
        let mut parts = vec![ContentPart::text(&prompt_text)];

        for image_path in images {
            let url = image_to_url(image_path)?;
            parts.push(ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url,
                    detail: detail.clone(),
                },
            });
        }

        Message::user_parts(parts)
    } else {
        Message::user(prompt_text)
    };

    let mut request = CompletionRequest::new(vec![message]);

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
    fn test_substitute_variables() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), Value::String("World".to_string()));
        vars.insert("count".to_string(), serde_json::json!(42));

        let template = "Hello {{ name }}! Count: {{ count }}";
        let result = substitute_variables(template, &vars);
        assert_eq!(result, "Hello World! Count: 42");
    }

    #[test]
    fn test_substitute_variables_missing() {
        let vars = HashMap::new();
        let template = "Hello {{ name }}!";
        let result = substitute_variables(template, &vars);
        assert_eq!(result, "Hello {{ name }}!"); // unchanged
    }

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
            images: None,
            image_detail: None,
        };
        assert_eq!(config.model, Some("gpt-4o".to_string()));
        assert_eq!(config.reasoning_level, Some(ReasoningLevel::High));
    }

    #[test]
    fn test_generate_config_with_images() {
        let config = GenerateConfig {
            images: Some(vec!["test.png".to_string()]),
            image_detail: Some("high".to_string()),
            ..Default::default()
        };
        assert_eq!(config.images, Some(vec!["test.png".to_string()]));
        assert_eq!(config.image_detail, Some("high".to_string()));
    }
}
