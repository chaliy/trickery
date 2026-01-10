use crate::commands::image::ImageResult;
use crate::provider::openai::OpenAIProvider;
use crate::provider::{
    ImageAction, ImageBackground, ImageFormat, ImageGenerationOptions, ImageQuality, ImageSize,
    Message, ResponsesRequest,
};
use crate::tools::ToolRegistry;
use crate::trickery::r#loop::{AgentLoop, LoopConfig};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

use super::generate::{substitute_variables, DEFAULT_MAX_ITERATIONS};

/// Configuration for image generation
#[derive(Debug, Clone, Default)]
pub struct ImageConfig {
    pub model: Option<String>,
    pub input_images: Option<Vec<String>>,
    pub size: Option<ImageSize>,
    pub quality: Option<ImageQuality>,
    pub output_format: Option<ImageFormat>,
    pub background: Option<ImageBackground>,
    pub action: Option<ImageAction>,
    pub compression: Option<u8>,
    /// Tool names for prompt pre-processing (optional)
    pub tool_names: Option<Vec<String>>,
    /// Max iterations for tool processing
    pub max_iterations: Option<u32>,
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

/// Process prompt with tools if specified, otherwise return as-is
async fn process_prompt_with_tools(
    prompt: &str,
    tool_names: &[String],
    max_iterations: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let provider = OpenAIProvider::from_env()?;
    let registry = ToolRegistry::with_builtins();

    let loop_config = LoopConfig {
        max_iterations,
        model: None, // Use default model for prompt processing
        reasoning_level: None,
        max_tokens: None,
    };

    // Create a prompt that asks the LLM to enhance the image prompt using tools
    let system_prompt = "You are helping to create an image generation prompt. Use the available tools to gather any information needed, then output ONLY the final image generation prompt text. Do not include any explanation or markdown formatting.";
    let messages = vec![Message::system(system_prompt), Message::user(prompt)];

    let agent = AgentLoop::new(provider, registry, loop_config);
    let result = agent.run(messages, tool_names).await?;

    Ok(result.content)
}

/// Generate image from template with variable substitution.
/// If tools are specified, the prompt is pre-processed with an agentic loop.
pub async fn generate_image(
    template: &str,
    input_variables: &HashMap<String, Value>,
    config: ImageConfig,
    output_path: &Path,
) -> Result<ImageResult, Box<dyn std::error::Error>> {
    // Substitute template variables
    let mut prompt = substitute_variables(template, input_variables);

    // If tools are specified, pre-process the prompt
    if let Some(ref tool_names) = config.tool_names {
        if !tool_names.is_empty() {
            let max_iterations = config.max_iterations.unwrap_or(DEFAULT_MAX_ITERATIONS);
            prompt = process_prompt_with_tools(&prompt, tool_names, max_iterations).await?;
        }
    }

    // Create provider
    let provider = OpenAIProvider::from_env()?;

    // Convert input images to URLs (base64 for local files)
    let input_images = if let Some(ref images) = config.input_images {
        let mut urls = Vec::new();
        for image_path in images {
            urls.push(image_to_url(image_path)?);
        }
        Some(urls)
    } else {
        None
    };

    // Build options
    let options = ImageGenerationOptions {
        size: config.size,
        quality: config.quality,
        output_format: config.output_format.clone(),
        background: config.background,
        action: config.action,
        compression: config.compression,
    };

    // Build request
    let mut request = ResponsesRequest::new(prompt).with_options(options);

    if let Some(model) = config.model {
        request = request.with_model(model);
    }

    if let Some(images) = input_images {
        request = request.with_images(images);
    }

    // Make API call
    let response = provider.create_response(request).await?;

    // Get first image result
    let image_result = response
        .images
        .into_iter()
        .next()
        .ok_or("No image generated in response")?;

    // Decode base64 and save to file
    let image_data = BASE64
        .decode(&image_result.result)
        .map_err(|e| format!("Failed to decode image data: {}", e))?;

    std::fs::write(output_path, &image_data).map_err(|e| {
        format!(
            "Failed to write image to '{}': {}",
            output_path.display(),
            e
        )
    })?;

    Ok(ImageResult {
        output_path: output_path.display().to_string(),
        revised_prompt: image_result.revised_prompt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

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
    fn test_image_to_url_nonexistent_file() {
        let result = image_to_url("/nonexistent/path/to/image.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_image_config_default() {
        let config = ImageConfig::default();
        assert!(config.model.is_none());
        assert!(config.input_images.is_none());
        assert!(config.size.is_none());
        assert!(config.quality.is_none());
    }
}
