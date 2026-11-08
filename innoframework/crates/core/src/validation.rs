//! Validation trait for invariant-carrying types.
//!
//! Types with invariants (Capability, Manifest, Session, RpcRoute, Tool, Agent,
//! ThemeTokens, BuildTarget per ARCHITECTURE_PLAN.md §2.3) implement `Validate`.
//! Validation runs at construction and before serialization, so the core never
//! processes unvalidated data.

use crate::error::{ErrorCode, InnoframeworkError};

/// Error code reserved for validation failures.
pub const VALIDATION_ERROR: ErrorCode = ErrorCode::new_const(1000);

/// A type whose invariants can be checked.
pub trait Validate {
    /// Returns `Ok(())` when every invariant holds, otherwise an error
    /// describing the first violated invariant.
    fn validate(&self) -> Result<(), InnoframeworkError>;
}

/// Helper: build a validation error with a user-facing message.
pub fn invalid(message: impl Into<String>) -> InnoframeworkError {
    InnoframeworkError::new(VALIDATION_ERROR, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Positive(i64);

    impl Validate for Positive {
        fn validate(&self) -> Result<(), InnoframeworkError> {
            if self.0 > 0 {
                Ok(())
            } else {
                Err(invalid("value must be positive"))
            }
        }
    }

    #[test]
    fn valid_value_passes() {
        assert!(Positive(1).validate().is_ok());
    }

    #[test]
    fn invalid_value_returns_validation_code() {
        let err = Positive(0).validate().unwrap_err();
        assert_eq!(err.code(), VALIDATION_ERROR);
        assert_eq!(err.user_message(), "value must be positive");
    }
}
