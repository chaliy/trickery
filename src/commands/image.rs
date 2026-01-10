use clap::{Args, ValueHint};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs::read_to_string;

use super::{CommandExec, CommandResult};
use crate::provider::{ImageAction, ImageBackground, ImageFormat, ImageQuality, ImageSize};
use crate::trickery::image::{generate_image, ImageConfig};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResult {
    pub output_path: String,
    pub revised_prompt: Option<String>,
}

impl CommandResult<ImageResult> for ImageResult {
    fn get_result(&self) -> &ImageResult {
        self
    }
}

fn parse_key_val(s: &str) -> Result<(String, Value), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=VALUE: no `=` found in `{}`", s))?;
    Ok((
        s[..pos].to_string(),
        Value::String(s[pos + 1..].to_string()),
    ))
}

fn parse_image_size(s: &str) -> Result<ImageSize, String> {
    s.parse()
}

fn parse_image_quality(s: &str) -> Result<ImageQuality, String> {
    s.parse()
}

fn parse_image_format(s: &str) -> Result<ImageFormat, String> {
    s.parse()
}

fn parse_image_background(s: &str) -> Result<ImageBackground, String> {
    s.parse()
}

fn parse_image_action(s: &str) -> Result<ImageAction, String> {
    s.parse()
}

/// Generate output filename with random suffix.
/// Uses input path stem if provided, otherwise defaults to "image".
/// E.g., "prompts/diagram.md" -> "diagram-a3f5x.png", or None -> "image-a3f5x.png"
fn generate_output_filename(input_path: Option<&Path>, format: Option<&ImageFormat>) -> PathBuf {
    let stem = input_path
        .and_then(|p| p.file_stem())
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let suffix: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    let ext = match format {
        Some(ImageFormat::Jpeg) => "jpg",
        Some(ImageFormat::Webp) => "webp",
        _ => "png",
    };

    PathBuf::from(format!("{}-{}.{}", stem, suffix.to_lowercase(), ext))
}

#[derive(Args)]
pub struct ImageArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub input: Option<PathBuf>,

    /// Direct text input (alternative to --input file)
    #[arg(short, long)]
    pub text: Option<String>,

    /// Output file path for the generated image (auto-generated if not provided)
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub save: Option<PathBuf>,

    /// Variables to be used in prompt
    #[arg(short, long="var", value_parser = parse_key_val, number_of_values = 1)]
    pub vars: Vec<(String, Value)>,

    /// Model to use (e.g., gpt-4.1, gpt-5, gpt-5.2)
    #[arg(short, long)]
    model: Option<String>,

    /// Input image files or URLs for editing (can be specified multiple times)
    #[arg(long)]
    image: Vec<String>,

    /// Image size: auto, 1024x1024, 1024x1536 (portrait), 1536x1024 (landscape)
    #[arg(long, value_parser = parse_image_size)]
    size: Option<ImageSize>,

    /// Image quality: auto, low, medium, high
    #[arg(long, value_parser = parse_image_quality)]
    quality: Option<ImageQuality>,

    /// Output format: png, jpeg, webp
    #[arg(long, value_parser = parse_image_format)]
    format: Option<ImageFormat>,

    /// Background: auto, transparent, opaque
    #[arg(long, value_parser = parse_image_background)]
    background: Option<ImageBackground>,

    /// Action: auto, generate, edit
    #[arg(long, value_parser = parse_image_action)]
    action: Option<ImageAction>,

    /// Compression level (0-100) for jpeg/webp formats
    #[arg(long)]
    compression: Option<u8>,
}

impl CommandExec<ImageResult> for ImageArgs {
    async fn exec(
        &self,
        context: &impl super::CommandExecutionContext,
    ) -> Result<Box<dyn CommandResult<ImageResult>>, Box<dyn std::error::Error>> {
        let template: String = match (&self.input, &self.text) {
            (Some(path), None) => read_to_string(path)
                .await
                .map_err(|e| format!("Failed to read input file '{}': {}", path.display(), e))?,
            (None, Some(text)) => text.clone(),
            (Some(_), Some(_)) => {
                return Err("Cannot specify both --input and --text".into());
            }
            (None, None) => {
                return Err("Either --input or --text is required".into());
            }
        };

        let input_variables: HashMap<String, Value> = self
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let config = ImageConfig {
            model: self.model.clone(),
            input_images: if self.image.is_empty() {
                None
            } else {
                Some(self.image.clone())
            },
            size: self.size.clone(),
            quality: self.quality.clone(),
            output_format: self.format.clone(),
            background: self.background.clone(),
            action: self.action.clone(),
            compression: self.compression,
        };

        // Use provided save path or auto-generate from input filename
        let output_path = match &self.save {
            Some(path) => path.clone(),
            None => generate_output_filename(self.input.as_deref(), self.format.as_ref()),
        };

        let result = generate_image(&template, &input_variables, config, &output_path).await?;

        if context.get_cli().is_interactive() {
            println!("Image saved to: {}", output_path.display());
            if let Some(ref revised) = result.revised_prompt {
                println!("Revised prompt: {}", revised);
            }
        }

        Ok(Box::from(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_val() {
        let (key, val) = parse_key_val("name=John").unwrap();
        assert_eq!(key, "name");
        assert_eq!(val, Value::String("John".to_string()));
    }

    #[test]
    fn test_parse_image_size() {
        assert_eq!(parse_image_size("auto").unwrap(), ImageSize::Auto);
        assert_eq!(parse_image_size("1024x1024").unwrap(), ImageSize::Square);
        assert_eq!(parse_image_size("1024x1536").unwrap(), ImageSize::Portrait);
        assert_eq!(parse_image_size("1536x1024").unwrap(), ImageSize::Landscape);
        assert!(parse_image_size("invalid").is_err());
    }

    #[test]
    fn test_parse_image_quality() {
        assert_eq!(parse_image_quality("auto").unwrap(), ImageQuality::Auto);
        assert_eq!(parse_image_quality("low").unwrap(), ImageQuality::Low);
        assert_eq!(parse_image_quality("medium").unwrap(), ImageQuality::Medium);
        assert_eq!(parse_image_quality("high").unwrap(), ImageQuality::High);
        assert!(parse_image_quality("invalid").is_err());
    }

    #[test]
    fn test_parse_image_format() {
        assert_eq!(parse_image_format("png").unwrap(), ImageFormat::Png);
        assert_eq!(parse_image_format("jpeg").unwrap(), ImageFormat::Jpeg);
        assert_eq!(parse_image_format("jpg").unwrap(), ImageFormat::Jpeg);
        assert_eq!(parse_image_format("webp").unwrap(), ImageFormat::Webp);
        assert!(parse_image_format("invalid").is_err());
    }

    #[test]
    fn test_parse_image_background() {
        assert_eq!(
            parse_image_background("auto").unwrap(),
            ImageBackground::Auto
        );
        assert_eq!(
            parse_image_background("transparent").unwrap(),
            ImageBackground::Transparent
        );
        assert_eq!(
            parse_image_background("opaque").unwrap(),
            ImageBackground::Opaque
        );
        assert!(parse_image_background("invalid").is_err());
    }

    #[test]
    fn test_parse_image_action() {
        assert_eq!(parse_image_action("auto").unwrap(), ImageAction::Auto);
        assert_eq!(
            parse_image_action("generate").unwrap(),
            ImageAction::Generate
        );
        assert_eq!(parse_image_action("edit").unwrap(), ImageAction::Edit);
        assert!(parse_image_action("invalid").is_err());
    }

    #[test]
    fn test_generate_output_filename_default_format() {
        let input = Path::new("prompts/diagram.md");
        let output = generate_output_filename(Some(input), None);
        let filename = output.to_str().unwrap();

        // Should start with stem from input
        assert!(filename.starts_with("diagram-"));
        // Should have 5-char random suffix and .png extension
        assert!(filename.ends_with(".png"));
        // Should match pattern: stem-xxxxx.png (total ~15 chars)
        assert_eq!(filename.len(), "diagram-xxxxx.png".len());
    }

    #[test]
    fn test_generate_output_filename_jpeg_format() {
        let input = Path::new("icon.md");
        let output = generate_output_filename(Some(input), Some(&ImageFormat::Jpeg));
        let filename = output.to_str().unwrap();

        assert!(filename.starts_with("icon-"));
        assert!(filename.ends_with(".jpg"));
    }

    #[test]
    fn test_generate_output_filename_webp_format() {
        let input = Path::new("test.md");
        let output = generate_output_filename(Some(input), Some(&ImageFormat::Webp));
        let filename = output.to_str().unwrap();

        assert!(filename.starts_with("test-"));
        assert!(filename.ends_with(".webp"));
    }

    #[test]
    fn test_generate_output_filename_uniqueness() {
        let input = Path::new("test.md");
        let output1 = generate_output_filename(Some(input), None);
        let output2 = generate_output_filename(Some(input), None);

        // Should generate different filenames (random suffix)
        assert_ne!(output1, output2);
    }

    #[test]
    fn test_generate_output_filename_no_input() {
        let output = generate_output_filename(None, None);
        let filename = output.to_str().unwrap();

        // Should use default "image" stem
        assert!(filename.starts_with("image-"));
        assert!(filename.ends_with(".png"));
        assert_eq!(filename.len(), "image-xxxxx.png".len());
    }

    #[test]
    fn test_generate_output_filename_no_input_webp() {
        let output = generate_output_filename(None, Some(&ImageFormat::Webp));
        let filename = output.to_str().unwrap();

        assert!(filename.starts_with("image-"));
        assert!(filename.ends_with(".webp"));
    }
}
