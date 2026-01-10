// Agentic loop - manages multi-turn tool-calling conversations.
// Design: Runs LLM calls in a loop, executing tool calls and feeding results back.
// Max iterations configurable (default 20) to prevent infinite loops.

use crate::provider::openai::OpenAIProvider;
use crate::provider::{CompletionRequest, Message, ReasoningLevel, ToolCall};
use crate::tools::{ToolError, ToolRegistry};

/// Configuration for the agentic loop
#[derive(Debug, Clone)]
pub struct LoopConfig {
    /// Maximum number of iterations (default: 20)
    pub max_iterations: u32,
    /// Model to use
    pub model: Option<String>,
    /// Reasoning level for o1/o3 models
    pub reasoning_level: Option<ReasoningLevel>,
    /// Maximum tokens per response
    pub max_tokens: Option<u32>,
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self {
            max_iterations: 20,
            model: None,
            reasoning_level: None,
            max_tokens: None,
        }
    }
}

/// Result of an agentic loop execution
#[derive(Debug)]
#[allow(dead_code)] // Fields are part of public API for future use
pub struct LoopResult {
    /// Final text content from the assistant
    pub content: String,
    /// Number of iterations completed
    pub iterations: u32,
    /// Tool calls that were executed
    pub tool_calls_executed: Vec<ExecutedToolCall>,
}

/// Record of an executed tool call
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields are part of public API for future use
pub struct ExecutedToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
    pub result: String,
}

/// Error during agentic loop execution
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)] // Variants are part of public API for future use
pub enum LoopError {
    #[error("Provider error: {0}")]
    Provider(#[from] crate::provider::ProviderError),
    #[error("Tool error: {0}")]
    Tool(#[from] ToolError),
    #[error("Max iterations ({0}) exceeded")]
    MaxIterationsExceeded(u32),
    #[error("No content in response")]
    NoContent,
}

/// Agentic loop executor
pub struct AgentLoop {
    provider: OpenAIProvider,
    registry: ToolRegistry,
    config: LoopConfig,
}

impl AgentLoop {
    /// Create new agent loop with provider, registry and config
    pub fn new(provider: OpenAIProvider, registry: ToolRegistry, config: LoopConfig) -> Self {
        Self {
            provider,
            registry,
            config,
        }
    }

    /// Set max iterations
    #[allow(dead_code)] // Builder method for public API
    pub fn with_max_iterations(mut self, max: u32) -> Self {
        self.config.max_iterations = max;
        self
    }

    /// Set model
    #[allow(dead_code)] // Builder method for public API
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.config.model = Some(model.into());
        self
    }

    /// Set reasoning level
    #[allow(dead_code)] // Builder method for public API
    pub fn with_reasoning_level(mut self, level: ReasoningLevel) -> Self {
        self.config.reasoning_level = Some(level);
        self
    }

    /// Set max tokens
    #[allow(dead_code)] // Builder method for public API
    pub fn with_max_tokens(mut self, max: u32) -> Self {
        self.config.max_tokens = Some(max);
        self
    }

    /// Run the agentic loop with initial messages and tools
    pub async fn run(
        &self,
        initial_messages: Vec<Message>,
        tool_names: &[String],
    ) -> Result<LoopResult, LoopError> {
        let mut messages = initial_messages;
        let mut iterations = 0u32;
        let mut tool_calls_executed = Vec::new();

        // Get tool definitions for requested tools
        let tools = if tool_names.is_empty() {
            self.registry.definitions()
        } else {
            self.registry.definitions_for(tool_names)
        };

        loop {
            iterations += 1;

            if iterations > self.config.max_iterations {
                return Err(LoopError::MaxIterationsExceeded(self.config.max_iterations));
            }

            // Build request
            let mut request = CompletionRequest::new(messages.clone());

            if let Some(ref model) = self.config.model {
                request = request.with_model(model.clone());
            }
            if let Some(level) = self.config.reasoning_level {
                request = request.with_reasoning_level(level);
            }
            if let Some(max_tokens) = self.config.max_tokens {
                request = request.with_max_tokens(max_tokens);
            }
            if !tools.is_empty() {
                request = request.with_tools(tools.clone());
            }

            // Make LLM call
            let response = self.provider.complete(request).await?;

            // Check if we have tool calls to execute
            if let Some(ref tool_calls) = response.tool_calls {
                if !tool_calls.is_empty() {
                    // Add assistant message with tool calls
                    messages.push(Message::assistant_with_tool_calls(tool_calls.clone()));

                    // Execute each tool call and add results
                    for tool_call in tool_calls {
                        let result = self.execute_tool_call(tool_call)?;
                        tool_calls_executed.push(result.clone());

                        // Add tool result message
                        messages.push(Message::tool_result(&tool_call.id, &result.result));
                    }

                    // Continue loop to get next response
                    continue;
                }
            }

            // No tool calls - we're done
            let content = response.content.unwrap_or_default();
            return Ok(LoopResult {
                content,
                iterations,
                tool_calls_executed,
            });
        }
    }

    /// Execute a single tool call
    fn execute_tool_call(&self, tool_call: &ToolCall) -> Result<ExecutedToolCall, LoopError> {
        let result = self
            .registry
            .execute(&tool_call.function.name, &tool_call.function.arguments)?;

        Ok(ExecutedToolCall {
            id: tool_call.id.clone(),
            name: tool_call.function.name.clone(),
            arguments: tool_call.function.arguments.clone(),
            result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_config_default() {
        let config = LoopConfig::default();
        assert_eq!(config.max_iterations, 20);
        assert!(config.model.is_none());
        assert!(config.reasoning_level.is_none());
        assert!(config.max_tokens.is_none());
    }

    #[test]
    fn test_executed_tool_call() {
        let call = ExecutedToolCall {
            id: "call_123".to_string(),
            name: "current_time".to_string(),
            arguments: "{}".to_string(),
            result: "2024-01-15T10:30:00Z".to_string(),
        };
        assert_eq!(call.name, "current_time");
    }

    #[test]
    fn test_loop_result() {
        let result = LoopResult {
            content: "Done".to_string(),
            iterations: 2,
            tool_calls_executed: vec![],
        };
        assert_eq!(result.content, "Done");
        assert_eq!(result.iterations, 2);
    }
}
