// OpenAI provider implementation.
// Env vars: OPENAI_API_KEY (required), OPENAI_BASE_URL (optional, default: https://api.openai.com/v1)

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FunctionCall, ProviderError,
    ReasoningLevel, Tool, ToolCall, Usage,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";
const DEFAULT_MODEL: &str = "gpt-5-mini";

/// OpenAI API client
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
    default_model: String,
}

impl OpenAIProvider {
    /// Create new provider from environment variables.
    /// OPENAI_API_KEY - required
    /// OPENAI_BASE_URL - optional (default: https://api.openai.com/v1)
    pub fn from_env() -> Result<Self, ProviderError> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| ProviderError::MissingApiKey("OPENAI_API_KEY".to_string()))?;
        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            default_model: DEFAULT_MODEL.to_string(),
        })
    }

    /// Create provider with explicit configuration (useful for testing)
    #[allow(dead_code)] // Used in tests and for manual configuration
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            default_model: DEFAULT_MODEL.to_string(),
        }
    }

    /// Create provider with custom client (for testing with mocks)
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_client(client: Client, api_key: String, base_url: String) -> Self {
        Self {
            client,
            api_key,
            base_url,
            default_model: DEFAULT_MODEL.to_string(),
        }
    }

    /// Complete a chat request
    pub async fn complete(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse, ProviderError> {
        let model = request.model.as_deref().unwrap_or(&self.default_model);
        let is_reasoning_model = model.starts_with("o1") || model.starts_with("o3");

        let mut api_request = OpenAIRequest {
            model: model.to_string(),
            messages: request
                .messages
                .iter()
                .map(OpenAIMessage::from_message)
                .collect(),
            tools: request
                .tools
                .as_ref()
                .map(|tools| tools.iter().map(OpenAITool::from_tool).collect()),
            max_completion_tokens: request.max_tokens,
            temperature: if is_reasoning_model {
                None
            } else {
                request.temperature
            },
            reasoning_effort: None,
        };

        // Add reasoning effort for o1/o3 models
        if is_reasoning_model {
            if let Some(level) = request.reasoning_level {
                api_request.reasoning_effort = Some(match level {
                    ReasoningLevel::Low => "low".to_string(),
                    ReasoningLevel::Medium => "medium".to_string(),
                    ReasoningLevel::High => "high".to_string(),
                });
            }
        }

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&api_request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: OpenAIResponse = response.json().await?;
        let choice = api_response.choices.into_iter().next().ok_or_else(|| {
            ProviderError::InvalidResponse("No choices in response".to_string())
        })?;

        Ok(CompletionResponse {
            content: choice.message.content,
            tool_calls: choice.message.tool_calls.map(|calls| {
                calls
                    .into_iter()
                    .map(|tc| ToolCall {
                        id: tc.id,
                        call_type: tc.type_field,
                        function: FunctionCall {
                            name: tc.function.name,
                            arguments: tc.function.arguments,
                        },
                    })
                    .collect()
            }),
            finish_reason: choice.finish_reason.unwrap_or_default(),
            usage: api_response
                .usage
                .map(|u| Usage {
                    prompt_tokens: u.prompt_tokens,
                    completion_tokens: u.completion_tokens,
                    total_tokens: u.total_tokens,
                })
                .unwrap_or_default(),
        })
    }
}

// OpenAI API request/response types

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_completion_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
}

/// OpenAI message with content as array of parts
#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<OpenAIContentPart>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

/// Content part in OpenAI format
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum OpenAIContentPart {
    Text { text: String },
    ImageUrl { image_url: OpenAIImageUrl },
}

#[derive(Debug, Serialize)]
struct OpenAIImageUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl OpenAIMessage {
    fn from_message(msg: &super::Message) -> Self {
        Self {
            role: match msg.role {
                super::Role::System => "system".to_string(),
                super::Role::User => "user".to_string(),
                super::Role::Assistant => "assistant".to_string(),
                super::Role::Tool => "tool".to_string(),
            },
            content: msg.content.as_ref().map(|parts| {
                parts
                    .iter()
                    .map(|p| match p {
                        ContentPart::Text { text } => OpenAIContentPart::Text {
                            text: text.clone(),
                        },
                        ContentPart::ImageUrl { image_url } => OpenAIContentPart::ImageUrl {
                            image_url: OpenAIImageUrl {
                                url: image_url.url.clone(),
                                detail: image_url.detail.clone(),
                            },
                        },
                    })
                    .collect()
            }),
            tool_calls: msg.tool_calls.as_ref().map(|calls| {
                calls
                    .iter()
                    .map(|tc| OpenAIToolCall {
                        id: tc.id.clone(),
                        type_field: tc.call_type.clone(),
                        function: OpenAIFunctionCall {
                            name: tc.function.name.clone(),
                            arguments: tc.function.arguments.clone(),
                        },
                    })
                    .collect()
            }),
            tool_call_id: msg.tool_call_id.clone(),
        }
    }
}

/// Response message (can have string content)
#[derive(Debug, Deserialize)]
struct OpenAIResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIToolCall {
    id: String,
    #[serde(rename = "type")]
    type_field: String,
    function: OpenAIFunctionCall,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    type_field: String,
    function: OpenAIFunctionDef,
}

impl OpenAITool {
    fn from_tool(tool: &Tool) -> Self {
        Self {
            type_field: tool.tool_type.clone(),
            function: OpenAIFunctionDef {
                name: tool.function.name.clone(),
                description: tool.function.description.clone(),
                parameters: tool.function.parameters.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct OpenAIFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_new() {
        let provider = OpenAIProvider::new("test-key".to_string(), None);
        assert_eq!(provider.api_key, "test-key");
        assert_eq!(provider.base_url, DEFAULT_BASE_URL);
        assert_eq!(provider.default_model, DEFAULT_MODEL);
    }

    #[test]
    fn test_provider_custom_base_url() {
        let provider = OpenAIProvider::new(
            "test-key".to_string(),
            Some("https://custom.api.com".to_string()),
        );
        assert_eq!(provider.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_openai_message_conversion() {
        let msg = super::super::Message::user("Hello");
        let openai_msg = OpenAIMessage::from_message(&msg);
        assert_eq!(openai_msg.role, "user");
        let content = openai_msg.content.unwrap();
        assert_eq!(content.len(), 1);
        match &content[0] {
            OpenAIContentPart::Text { text } => assert_eq!(text, "Hello"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_openai_message_serialization() {
        let msg = super::super::Message::user("Hello world");
        let openai_msg = OpenAIMessage::from_message(&msg);
        let json = serde_json::to_string(&openai_msg).unwrap();
        assert!(json.contains(r#""type":"text""#));
        assert!(json.contains(r#""text":"Hello world""#));
    }

    #[test]
    fn test_openai_tool_conversion() {
        let tool = Tool::function(
            "get_weather",
            "Get weather for a location",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                }
            }),
        );
        let openai_tool = OpenAITool::from_tool(&tool);
        assert_eq!(openai_tool.type_field, "function");
        assert_eq!(openai_tool.function.name, "get_weather");
    }

    #[tokio::test]
    async fn test_complete_mock() {
        use mockito::Server;

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "choices": [{
                        "message": {
                            "role": "assistant",
                            "content": "Hello! How can I help you?"
                        },
                        "finish_reason": "stop"
                    }],
                    "usage": {
                        "prompt_tokens": 10,
                        "completion_tokens": 8,
                        "total_tokens": 18
                    }
                }"#,
            )
            .create_async()
            .await;

        let provider = OpenAIProvider::new("test-key".to_string(), Some(server.url()));

        let request = CompletionRequest::new(vec![super::super::Message::user("Hi")]);
        let response = provider.complete(request).await.unwrap();

        assert_eq!(
            response.content,
            Some("Hello! How can I help you?".to_string())
        );
        assert_eq!(response.finish_reason, "stop");
        assert_eq!(response.usage.total_tokens, 18);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_complete_with_tools_mock() {
        use mockito::Server;

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "choices": [{
                        "message": {
                            "role": "assistant",
                            "content": null,
                            "tool_calls": [{
                                "id": "call_abc123",
                                "type": "function",
                                "function": {
                                    "name": "get_weather",
                                    "arguments": "{\"location\": \"Paris\"}"
                                }
                            }]
                        },
                        "finish_reason": "tool_calls"
                    }],
                    "usage": {
                        "prompt_tokens": 20,
                        "completion_tokens": 15,
                        "total_tokens": 35
                    }
                }"#,
            )
            .create_async()
            .await;

        let provider = OpenAIProvider::new("test-key".to_string(), Some(server.url()));

        let tool = Tool::function(
            "get_weather",
            "Get weather",
            serde_json::json!({"type": "object"}),
        );
        let request = CompletionRequest::new(vec![super::super::Message::user(
            "What's the weather in Paris?",
        )])
        .with_tools(vec![tool]);

        let response = provider.complete(request).await.unwrap();

        assert!(response.content.is_none());
        let tool_calls = response.tool_calls.unwrap();
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].function.name, "get_weather");
        assert_eq!(response.finish_reason, "tool_calls");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_api_error_handling() {
        use mockito::Server;

        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(401)
            .with_body(r#"{"error": {"message": "Invalid API key"}}"#)
            .create_async()
            .await;

        let provider = OpenAIProvider::new("invalid-key".to_string(), Some(server.url()));

        let request = CompletionRequest::new(vec![super::super::Message::user("Hi")]);
        let result = provider.complete(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ProviderError::Api { status, .. } => assert_eq!(status, 401),
            _ => panic!("Expected Api error"),
        }

        mock.assert_async().await;
    }
}
