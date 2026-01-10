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
    let data = std::fs::read(path)
        .map_err(|e| format!("Failed to read image file '{}': {}", image_path, e))?;

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
    use std::io::Write;
    use tempfile::NamedTempFile;

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
            model: Some("gpt-5.2".to_string()),
            reasoning_level: Some(ReasoningLevel::High),
            tools: None,
            max_tokens: Some(1000),
            images: None,
            image_detail: None,
        };
        assert_eq!(config.model, Some("gpt-5.2".to_string()));
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

    // Image URL tests
    #[test]
    fn test_image_to_url_http_passthrough() {
        let url = "http://example.com/image.png";
        let result = image_to_url(url).unwrap();
        assert_eq!(result, url);
    }

    #[test]
    fn test_image_to_url_https_passthrough() {
        let url = "https://example.com/path/to/image.jpg";
        let result = image_to_url(url).unwrap();
        assert_eq!(result, url);
    }

    #[test]
    fn test_image_to_url_local_png() {
        let mut file = NamedTempFile::with_suffix(".png").unwrap();
        let test_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG magic bytes
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/png;base64,"));
        // Verify the base64 content decodes correctly
        let base64_part = result.strip_prefix("data:image/png;base64,").unwrap();
        let decoded = BASE64.decode(base64_part).unwrap();
        assert_eq!(decoded, test_data);
    }

    #[test]
    fn test_image_to_url_local_jpeg() {
        let mut file = NamedTempFile::with_suffix(".jpg").unwrap();
        let test_data = vec![0xFF, 0xD8, 0xFF]; // JPEG magic bytes
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn test_image_to_url_local_jpeg_extension() {
        let mut file = NamedTempFile::with_suffix(".jpeg").unwrap();
        let test_data = vec![0xFF, 0xD8, 0xFF];
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/jpeg;base64,"));
    }

    #[test]
    fn test_image_to_url_local_gif() {
        let mut file = NamedTempFile::with_suffix(".gif").unwrap();
        let test_data = vec![0x47, 0x49, 0x46, 0x38]; // GIF magic bytes
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/gif;base64,"));
    }

    #[test]
    fn test_image_to_url_local_webp() {
        let mut file = NamedTempFile::with_suffix(".webp").unwrap();
        let test_data = vec![0x52, 0x49, 0x46, 0x46]; // RIFF header
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/webp;base64,"));
    }

    #[test]
    fn test_image_to_url_unknown_extension_defaults_to_png() {
        let mut file = NamedTempFile::with_suffix(".unknown").unwrap();
        let test_data = vec![0x00, 0x01, 0x02];
        file.write_all(&test_data).unwrap();

        let result = image_to_url(file.path().to_str().unwrap()).unwrap();
        assert!(result.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_image_to_url_nonexistent_file() {
        let result = image_to_url("/nonexistent/path/to/image.png");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("/nonexistent/path/to/image.png"),
            "Error should contain file path: {}",
            err
        );
        assert!(
            err.contains("Failed to read image file"),
            "Error should indicate image file failure: {}",
            err
        );
    }

    // Multimodal message construction tests
    #[test]
    fn test_multimodal_message_with_image_url() {
        let parts = vec![
            ContentPart::text("Describe this image"),
            ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url: "https://example.com/image.png".to_string(),
                    detail: Some("high".to_string()),
                },
            },
        ];
        let message = Message::user_parts(parts);

        assert_eq!(message.role, crate::provider::Role::User);
        let content = message.content.unwrap();
        assert_eq!(content.len(), 2);

        match &content[0] {
            ContentPart::Text { text } => assert_eq!(text, "Describe this image"),
            _ => panic!("Expected text part"),
        }

        match &content[1] {
            ContentPart::ImageUrl { image_url } => {
                assert_eq!(image_url.url, "https://example.com/image.png");
                assert_eq!(image_url.detail, Some("high".to_string()));
            }
            _ => panic!("Expected image URL part"),
        }
    }

    #[test]
    fn test_multimodal_message_multiple_images() {
        let parts = vec![
            ContentPart::text("Compare these images"),
            ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url: "https://example.com/image1.png".to_string(),
                    detail: Some("auto".to_string()),
                },
            },
            ContentPart::ImageUrl {
                image_url: ImageUrl {
                    url: "https://example.com/image2.png".to_string(),
                    detail: Some("auto".to_string()),
                },
            },
        ];
        let message = Message::user_parts(parts);

        let content = message.content.unwrap();
        assert_eq!(content.len(), 3);
    }
}
