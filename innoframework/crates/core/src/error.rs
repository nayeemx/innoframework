//! innoframework-core: error model with stable error codes.
//!
//! Every error in the framework is an `InnoframeworkError` with:
//! - A stable error code (AUD-xxxx format) for programmatic handling
//! - A user-facing message (no internal details, no secrets)
//! - An internal details field (for logs and debugging, never transmitted)

use std::fmt;
use thiserror::Error;

/// Stable error code format: AUD-NNNN where NNNN is a 4-digit number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode(u16);

impl ErrorCode {
    /// Create a new error code. Panics if the code is 0 (reserved).
    pub fn new(code: u16) -> Self {
        assert!(code > 0, "error code 0 is reserved");
        Self(code)
    }

    /// Create an error code in a `const` context. Panics at compile time if
    /// `code` is 0. Crate-private: public codes are created with `new`.
    pub(crate) const fn new_const(code: u16) -> Self {
        assert!(code > 0, "error code 0 is reserved");
        Self(code)
    }

    /// Parse from a string like "AUD-0001". Returns None if the format is invalid.
    pub fn parse(s: &str) -> Option<Self> {
        if !s.starts_with("AUD-") {
            return None;
        }
        let code = s[4..].parse::<u16>().ok()?;
        if code == 0 {
            return None;
        }
        Some(Self::new(code))
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AUD-{:04}", self.0)
    }
}

/// The framework error type. Every error has a code, a user message, and optional internal details.
#[derive(Debug, Clone, Error)]
pub struct InnoframeworkError {
    code: ErrorCode,
    user_message: String,
    internal_details: Option<String>,
}

impl InnoframeworkError {
    /// Create a new error with a code and user-facing message.
    /// Internal details are None by default.
    pub fn new(code: ErrorCode, user_message: impl Into<String>) -> Self {
        Self {
            code,
            user_message: user_message.into(),
            internal_details: None,
        }
    }

    /// Add internal details (for logs; never sent to clients).
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.internal_details = Some(details.into());
        self
    }

    /// Get the error code.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Get the user-facing message.
    pub fn user_message(&self) -> &str {
        &self.user_message
    }

    /// Get the internal details, if any.
    pub fn internal_details(&self) -> Option<&str> {
        self.internal_details.as_deref()
    }
}

impl fmt::Display for InnoframeworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.user_message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_code_format() {
        let code = ErrorCode::new(1);
        assert_eq!(code.to_string(), "AUD-0001");

        let code = ErrorCode::new(9999);
        assert_eq!(code.to_string(), "AUD-9999");
    }

    #[test]
    fn error_code_parse() {
        assert_eq!(ErrorCode::parse("AUD-0001"), Some(ErrorCode::new(1)));
        assert_eq!(ErrorCode::parse("AUD-9999"), Some(ErrorCode::new(9999)));
        assert_eq!(ErrorCode::parse("AUD-0000"), None); // 0 is reserved
        assert_eq!(ErrorCode::parse("AUD"), None);
        assert_eq!(ErrorCode::parse("ERR-0001"), None);
    }

    #[test]
    fn error_with_no_secrets() {
        let err = InnoframeworkError::new(ErrorCode::new(1), "Invalid input")
            .with_details("Input was: password123");
        assert!(!err.user_message().contains("password"));
        assert!(err.internal_details().unwrap().contains("password"));
    }

    #[test]
    fn error_display() {
        let err = InnoframeworkError::new(ErrorCode::new(42), "Something went wrong");
        assert_eq!(err.to_string(), "AUD-0042: Something went wrong");
    }
}
