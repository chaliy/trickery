use clap::{Args, ValueHint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
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

#[derive(Args)]
pub struct ImageArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,

    /// Output file path for the generated image
    #[arg(long = "out", value_hint = ValueHint::FilePath)]
    out_file: PathBuf,

    /// Variables to be used in prompt
    #[arg(short, long="var", value_parser = parse_key_val, number_of_values = 1)]
    vars: Vec<(String, Value)>,

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
        let input_path = match &self.input {
            Some(path) => path,
            None => return Err("Input file path is required".into()),
        };

        let input_variables: HashMap<String, Value> = self
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let template: String = read_to_string(input_path).await.map_err(|e| {
            format!(
                "Failed to read input file '{}': {}",
                input_path.display(),
                e
            )
        })?;

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

        let result = generate_image(&template, &input_variables, config, &self.out_file).await?;

        if context.get_cli().is_interactive() {
            println!("Image saved to: {}", self.out_file.display());
            if let Some(ref revised) = result.revised_prompt {
                println!("Revised prompt: {}", revised);
            }
        };

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
}
