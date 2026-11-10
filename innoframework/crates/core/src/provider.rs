//! Model provider trait and mock implementation (TR-5.2).
//!
//! All model providers sit behind this trait so the agent executor never
//! depends on a concrete binding (llama.cpp, Gemini, OpenAI). The mock
//! provider is used for tests and dev; real providers are added in later
//! phases behind feature gates.

use crate::error::{ErrorCode, InnoframeworkError};

pub const PROVIDER_ERROR: ErrorCode = ErrorCode::new_const(1900);

/// A model provider generates text from a prompt + system template.
///
/// Implementations are responsible for their own transport (local child
/// process for llama.cpp, HTTP for cloud providers). API keys are never
/// passed through this trait — they are held by the concrete implementation.
pub trait ModelProvider: Send + Sync {
    /// Generate a completion. `system_template` may contain `{{placeholder}}`
    /// substitution only — no raw user input concatenation (AI-14).
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
        system_template: &'a str,
    ) -> BoxFuture<'a, Result<String, InnoframeworkError>>;

    /// Human-readable provider name (for logging, never for user display).
    fn name(&self) -> &'static str;
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// Mock provider for tests and dev. Returns a canned response based on
/// keywords in the prompt. Never calls an external service.
#[derive(Debug, Default, Clone)]
pub struct MockModelProvider;

impl ModelProvider for MockModelProvider {
    fn generate<'a>(
        &'a self,
        prompt: &'a str,
        system_template: &'a str,
    ) -> BoxFuture<'a, Result<String, InnoframeworkError>> {
        Box::pin(async move {
            let lower = prompt.to_lowercase();
            let response = if lower.contains("tool") && !lower.contains("contents of") {
                format!(
                    "TOOL_CALL:{{\"name\":\"read_file\",\"args\":{{\"path\":\"{}\"}}}}",
                    prompt
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '/')
                        .take(20)
                        .collect::<String>()
                )
            } else if lower.contains("code") || lower.contains("refactor") {
                "```rust\nfn example() -> i32 {\n    42\n}\n```".to_string()
            } else if lower.contains("sum") || lower.contains("calculate") {
                "The sum is 42.".to_string()
            } else if lower.contains("error") || lower.contains("fail") {
                "I cannot help with that request.".to_string()
            } else {
                format!(
                    "Mock response to: {} (system: {})",
                    prompt.chars().take(40).collect::<String>(),
                    system_template.chars().take(20).collect::<String>()
                )
            };
            Ok(response)
        })
    }

    fn name(&self) -> &'static str {
        "mock"
    }
}

/// Substitute `{{key}}` placeholders in a template with values from `vars`.
/// Only keys present in `vars` are substituted; unknown keys are left intact.
/// This prevents raw user input concatenation into system prompts (AI-14).
pub fn substitute_template(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_provider_returns_response() {
        let provider = MockModelProvider;
        let out = provider
            .generate("Write some code", "You are a coding assistant.")
            .await
            .unwrap();
        assert!(!out.is_empty());
        assert_eq!(provider.name(), "mock");
    }

    #[test]
    fn substitute_template_replaces_placeholders() {
        let t = "Hello {{name}}, your role is {{role}}.";
        let out = substitute_template(t, &[("name", "Ada"), ("role", "admin")]);
        assert_eq!(out, "Hello Ada, your role is admin.");
    }

    #[test]
    fn substitute_template_leaves_unknown_keys() {
        let t = "Hello {{name}}.";
        let out = substitute_template(t, &[("name", "Ada")]);
        assert_eq!(out, "Hello Ada.");
    }

    #[test]
    fn substitute_template_no_placeholders() {
        let t = "Plain text.";
        let out = substitute_template(t, &[]);
        assert_eq!(out, "Plain text.");
    }
}
