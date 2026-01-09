// Provider abstraction for LLM backends (OpenAI, Anthropic, Gemini).
// Design: Each provider implements the Provider trait with its own client.
// Note: Provider only handles API contract, no template processing.

pub mod openai;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("API key not found: {0}")]
    MissingApiKey(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

/// Reasoning effort level for models that support it (o1, o3, etc.)
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningLevel {
    Low,
    #[default]
    Medium,
    High,
}

impl std::str::FromStr for ReasoningLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(format!(
                "Invalid reasoning level: {s}. Use: low, medium, high"
            )),
        }
    }
}

/// Message role in conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// Content part in a message (OpenAI format)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    #[allow(dead_code)] // For future image support
    ImageUrl { image_url: ImageUrl },
}

/// Image URL for vision models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl ContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    #[allow(dead_code)] // For future image support
    pub fn image_url(url: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: ImageUrl {
                url: url.into(),
                detail: None,
            },
        }
    }
}

/// A message in the conversation (OpenAI format with content parts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ContentPart>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    #[allow(dead_code)] // Part of public API for future providers
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: Some(vec![ContentPart::text(content)]),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: Some(vec![ContentPart::text(content)]),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    /// Create user message with multiple content parts
    #[allow(dead_code)] // Part of public API for future providers
    pub fn user_parts(parts: Vec<ContentPart>) -> Self {
        Self {
            role: Role::User,
            content: Some(parts),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    #[allow(dead_code)] // Part of public API for future providers
    pub fn tool_result(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: Some(vec![ContentPart::text(content)]),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }

    /// Get text content as string (concatenates all text parts)
    #[allow(dead_code)]
    pub fn text_content(&self) -> Option<String> {
        self.content.as_ref().map(|parts| {
            parts
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("")
        })
    }
}

/// Tool call made by assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

/// Function call details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDef,
}

impl Tool {
    #[allow(dead_code)] // Part of public API for future providers
    pub fn function(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}

/// Function definition for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Request configuration for completion
#[derive(Debug, Clone, Default)]
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub model: Option<String>,
    pub reasoning_level: Option<ReasoningLevel>,
    pub tools: Option<Vec<Tool>>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

impl CompletionRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_reasoning_level(mut self, level: ReasoningLevel) -> Self {
        self.reasoning_level = Some(level);
        self
    }

    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    #[allow(dead_code)] // Part of public API for future providers
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
}

/// Response from completion
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are part of public API
pub struct CompletionResponse {
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: String,
    pub usage: Usage,
}

/// Token usage info
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Fields are part of public API
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_level_from_str() {
        assert_eq!(
            "low".parse::<ReasoningLevel>().unwrap(),
            ReasoningLevel::Low
        );
        assert_eq!(
            "MEDIUM".parse::<ReasoningLevel>().unwrap(),
            ReasoningLevel::Medium
        );
        assert_eq!(
            "High".parse::<ReasoningLevel>().unwrap(),
            ReasoningLevel::High
        );
        assert!("invalid".parse::<ReasoningLevel>().is_err());
    }

    #[test]
    fn test_content_part_text() {
        let part = ContentPart::text("Hello");
        assert_eq!(part, ContentPart::Text { text: "Hello".to_string() });
    }

    #[test]
    fn test_message_constructors() {
        let sys = Message::system("You are helpful");
        assert_eq!(sys.role, Role::System);
        assert_eq!(sys.text_content(), Some("You are helpful".to_string()));

        let user = Message::user("Hello");
        assert_eq!(user.role, Role::User);
        assert_eq!(user.text_content(), Some("Hello".to_string()));

        let tool = Message::tool_result("call_123", "result");
        assert_eq!(tool.role, Role::Tool);
        assert_eq!(tool.tool_call_id, Some("call_123".to_string()));
    }

    #[test]
    fn test_message_content_parts() {
        let msg = Message::user_parts(vec![
            ContentPart::text("Hello "),
            ContentPart::text("World"),
        ]);
        assert_eq!(msg.text_content(), Some("Hello World".to_string()));
    }

    #[test]
    fn test_content_part_serialization() {
        let part = ContentPart::text("Hello");
        let json = serde_json::to_string(&part).unwrap();
        assert_eq!(json, r#"{"type":"text","text":"Hello"}"#);
    }
}
