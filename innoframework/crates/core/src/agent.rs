//! Agent model (ARCHITECTURE_PLAN.md §2.3).
//!
//! Every agent has explicit limits (no unbounded agents). Tools must be
//! in the registry (allowlist check). Each bound is enforced at construction.

use crate::error::{ErrorCode, InnoframeworkError};
use crate::tool::ToolRegistry;
use crate::validation::{invalid, Validate};

/// Error code for agent validation failures.
pub const AGENT_ERROR: ErrorCode = ErrorCode::new_const(1400);

/// Hard limits on an agent's execution. Mandatory — no unbounded agents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentLimits {
    pub max_steps: u32,
    pub timeout_ms: u64,
    pub max_output_chars: usize,
}

impl Validate for AgentLimits {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        if self.max_steps == 0 {
            return Err(invalid("max_steps must be > 0"));
        }
        if self.timeout_ms == 0 {
            return Err(invalid("timeout_ms must be > 0"));
        }
        if self.max_output_chars == 0 {
            return Err(invalid("max_output_chars must be > 0"));
        }
        Ok(())
    }
}

/// An agent definition.
#[derive(Debug, Clone)]
pub struct Agent {
    model_ref: String,
    /// Names of tools this agent may call (subset of the registry).
    tool_names: Vec<String>,
    system_template: String,
    limits: AgentLimits,
}

impl Agent {
    /// Create an agent. Verifies that every named tool exists in `registry`.
    pub fn new(
        model_ref: impl Into<String>,
        tool_names: Vec<String>,
        system_template: impl Into<String>,
        limits: AgentLimits,
        registry: &ToolRegistry,
    ) -> Result<Self, InnoframeworkError> {
        let agent = Self {
            model_ref: model_ref.into(),
            tool_names,
            system_template: system_template.into(),
            limits,
        };
        agent.validate()?;
        agent.validate_against_registry(registry)?;
        Ok(agent)
    }

    fn validate_against_registry(&self, registry: &ToolRegistry) -> Result<(), InnoframeworkError> {
        for name in &self.tool_names {
            if registry.get(name).is_none() {
                return Err(InnoframeworkError::new(
                    AGENT_ERROR,
                    format!("tool {name} is not in the registry"),
                ));
            }
        }
        Ok(())
    }

    pub fn model_ref(&self) -> &str {
        &self.model_ref
    }

    pub fn tool_names(&self) -> &[String] {
        &self.tool_names
    }

    pub fn system_template(&self) -> &str {
        &self.system_template
    }

    pub fn limits(&self) -> &AgentLimits {
        &self.limits
    }
}

impl Validate for Agent {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        if self.model_ref.is_empty() {
            return Err(invalid("model_ref cannot be empty"));
        }
        self.limits.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{RiskLevel, Tool, ToolRegistry};

    fn registry_with(names: &[&str]) -> ToolRegistry {
        let mut reg = ToolRegistry::new();
        for &n in names {
            reg.register(Tool::new(n, "desc", RiskLevel::Low, "i", "o").unwrap())
                .unwrap();
        }
        reg
    }

    fn limits() -> AgentLimits {
        AgentLimits {
            max_steps: 10,
            timeout_ms: 5000,
            max_output_chars: 4096,
        }
    }

    #[test]
    fn valid_agent_passes() {
        let reg = registry_with(&["read_file", "search"]);
        let a = Agent::new(
            "llama-3b",
            vec!["read_file".to_string(), "search".to_string()],
            "You are a helpful assistant.",
            limits(),
            &reg,
        )
        .unwrap();
        assert_eq!(a.tool_names().len(), 2);
    }

    #[test]
    fn unregistered_tool_is_rejected() {
        let reg = registry_with(&["read_file"]);
        let err = Agent::new(
            "llama-3b",
            vec!["exec".to_string()], // not in registry
            "tmpl",
            limits(),
            &reg,
        )
        .unwrap_err();
        assert_eq!(err.code(), AGENT_ERROR);
        assert!(err.user_message().contains("not in the registry"));
    }

    #[test]
    fn zero_max_steps_rejected() {
        let lim = AgentLimits {
            max_steps: 0,
            timeout_ms: 5000,
            max_output_chars: 4096,
        };
        assert!(lim.validate().is_err());
    }

    #[test]
    fn zero_timeout_rejected() {
        let lim = AgentLimits {
            max_steps: 10,
            timeout_ms: 0,
            max_output_chars: 4096,
        };
        assert!(lim.validate().is_err());
    }

    #[test]
    fn zero_output_chars_rejected() {
        let lim = AgentLimits {
            max_steps: 10,
            timeout_ms: 5000,
            max_output_chars: 0,
        };
        assert!(lim.validate().is_err());
    }

    #[test]
    fn empty_model_ref_rejected() {
        let reg = registry_with(&[]);
        assert!(Agent::new("", vec![], "tmpl", limits(), &reg).is_err());
    }

    #[test]
    fn agent_with_no_tools_is_valid() {
        let reg = registry_with(&[]);
        assert!(Agent::new("llama-3b", vec![], "tmpl", limits(), &reg).is_ok());
    }
}
