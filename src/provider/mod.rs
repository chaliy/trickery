// Provider abstraction for LLM backends (OpenAI, Anthropic, Gemini).
// Design: Each provider implements the Provider trait with its own client.

pub mod openai;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,
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
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    #[allow(dead_code)] // Part of public API for future providers
    pub fn tool_result(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
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

/// Template variable substitution
pub fn substitute_variables(
    template: &str,
    variables: &HashMap<String, serde_json::Value>,
) -> String {
    let mut result = template.to_string();
    for (key, value) in variables {
        let placeholder = format!("{{{{ {} }}}}", key);
        let replacement = match value {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        result = result.replace(&placeholder, &replacement);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_variables() {
        let mut vars = HashMap::new();
        vars.insert(
            "name".to_string(),
            serde_json::Value::String("World".to_string()),
        );
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
    fn test_message_constructors() {
        let sys = Message::system("You are helpful");
        assert_eq!(sys.role, Role::System);
        assert_eq!(sys.content, Some("You are helpful".to_string()));

        let user = Message::user("Hello");
        assert_eq!(user.role, Role::User);
        assert_eq!(user.content, Some("Hello".to_string()));

        let tool = Message::tool_result("call_123", "result");
        assert_eq!(tool.role, Role::Tool);
        assert_eq!(tool.tool_call_id, Some("call_123".to_string()));
    }
}
