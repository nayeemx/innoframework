//! Tool model (ARCHITECTURE_PLAN.md §2.3).
//!
//! Agent tools go through an allowlist registry. The registry rejects
//! duplicate names. A `High` risk level requires explicit opt-in.

use crate::error::{ErrorCode, InnoframeworkError};
use crate::validation::{invalid, Validate};

/// Error code for tool registration failures.
pub const TOOL_ERROR: ErrorCode = ErrorCode::new_const(1300);

/// Risk level of a tool. High-risk tools require explicit opt-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// A tool that an agent can call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    name: String,
    description: String,
    risk_level: RiskLevel,
    input_schema: String,
    output_schema: String,
}

impl Tool {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        risk_level: RiskLevel,
        input_schema: impl Into<String>,
        output_schema: impl Into<String>,
    ) -> Result<Self, InnoframeworkError> {
        let tool = Self {
            name: name.into(),
            description: description.into(),
            risk_level,
            input_schema: input_schema.into(),
            output_schema: output_schema.into(),
        };
        tool.validate()?;
        Ok(tool)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn risk_level(&self) -> RiskLevel {
        self.risk_level
    }

    pub fn input_schema(&self) -> &str {
        &self.input_schema
    }

    pub fn output_schema(&self) -> &str {
        &self.output_schema
    }
}

impl Validate for Tool {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        if self.name.is_empty() {
            return Err(invalid("tool name cannot be empty"));
        }
        if self.description.is_empty() {
            return Err(invalid(format!("tool {} has no description", self.name)));
        }
        if self.input_schema.is_empty() || self.output_schema.is_empty() {
            return Err(invalid(format!(
                "tool {} must have input and output schemas",
                self.name
            )));
        }
        Ok(())
    }
}

/// Allowlist-only tool registry. Rejects duplicate names.
#[derive(Debug, Default)]
pub struct ToolRegistry {
    tools: Vec<Tool>,
    high_risk_opted_in: bool,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow high-risk tools to be registered.
    pub fn allow_high_risk(&mut self) {
        self.high_risk_opted_in = true;
    }

    /// Register a tool. Rejects duplicates and high-risk tools without opt-in.
    pub fn register(&mut self, tool: Tool) -> Result<(), InnoframeworkError> {
        // Validate the tool itself.
        tool.validate()?;

        // Reject duplicates.
        if self.tools.iter().any(|t| t.name == tool.name) {
            return Err(InnoframeworkError::new(
                TOOL_ERROR,
                format!("tool {} is already registered", tool.name),
            ));
        }

        // Reject high-risk tools unless opted in.
        if tool.risk_level == RiskLevel::High && !self.high_risk_opted_in {
            return Err(InnoframeworkError::new(
                TOOL_ERROR,
                format!("tool {} is high-risk; explicit opt-in required", tool.name),
            ));
        }

        self.tools.push(tool);
        Ok(())
    }

    /// Look up a tool by name (allowlist check).
    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.name == name)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn low_risk_tool(name: &str) -> Tool {
        Tool::new(name, "desc", RiskLevel::Low, "in_schema", "out_schema").unwrap()
    }

    fn high_risk_tool(name: &str) -> Tool {
        Tool::new(name, "desc", RiskLevel::High, "in_schema", "out_schema").unwrap()
    }

    #[test]
    fn register_and_lookup_low_risk() {
        let mut reg = ToolRegistry::new();
        reg.register(low_risk_tool("read_file")).unwrap();
        assert_eq!(reg.len(), 1);
        assert!(reg.get("read_file").is_some());
        assert!(reg.get("write_file").is_none());
    }

    #[test]
    fn duplicate_name_rejected() {
        let mut reg = ToolRegistry::new();
        reg.register(low_risk_tool("read_file")).unwrap();
        let err = reg.register(low_risk_tool("read_file")).unwrap_err();
        assert_eq!(err.code(), TOOL_ERROR);
        assert!(err.user_message().contains("already registered"));
    }

    #[test]
    fn high_risk_rejected_without_opt_in() {
        let mut reg = ToolRegistry::new();
        let err = reg.register(high_risk_tool("exec")).unwrap_err();
        assert_eq!(err.code(), TOOL_ERROR);
        assert!(err.user_message().contains("high-risk"));
    }

    #[test]
    fn high_risk_allowed_with_opt_in() {
        let mut reg = ToolRegistry::new();
        reg.allow_high_risk();
        reg.register(high_risk_tool("exec")).unwrap();
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn empty_name_rejected() {
        assert!(Tool::new("", "d", RiskLevel::Low, "i", "o").is_err());
    }

    #[test]
    fn empty_description_rejected() {
        assert!(Tool::new("t", "", RiskLevel::Low, "i", "o").is_err());
    }

    #[test]
    fn empty_schema_rejected() {
        assert!(Tool::new("t", "d", RiskLevel::Low, "", "o").is_err());
        assert!(Tool::new("t", "d", RiskLevel::Low, "i", "").is_err());
    }
}
