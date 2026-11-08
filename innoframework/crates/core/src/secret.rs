//! Secret boundary check (ARCHITECTURE_PLAN.md §2.3, §3 "Fail closed").
//!
//! The manifest is committed to source control and read by the build, so it
//! must never contain credentials. Every manifest string value is scanned
//! before any other validation. A match is a hard error.
//!
//! Security rules for this module:
//! - Error messages name the field and the matched rule, never the value.
//! - Detection is conservative: it targets well-known credential formats and
//!   high-entropy assignments under secret-like keys. It is not a complete
//!   DLP scanner, and the docs must say so.

use crate::error::{ErrorCode, InnoframeworkError};

/// Error code for a secret found in a manifest value.
pub const SECRET_IN_MANIFEST: ErrorCode = ErrorCode::new_const(1250);

/// Known credential prefixes. A value starting with one of these is rejected.
const SECRET_PREFIXES: &[(&str, &str)] = &[
    ("AKIA", "AWS access key id"),
    ("ASIA", "AWS temporary access key id"),
    ("ghp_", "GitHub personal access token"),
    ("gho_", "GitHub OAuth token"),
    ("ghs_", "GitHub server token"),
    ("github_pat_", "GitHub fine-grained token"),
    ("sk_live_", "Stripe live secret key"),
    ("rk_live_", "Stripe live restricted key"),
    ("xoxb-", "Slack bot token"),
    ("xoxp-", "Slack user token"),
    ("sk-ant-", "Anthropic API key"),
    ("-----BEGIN ", "PEM private key block"),
    ("eyJhbGciOi", "JWT"),
];

/// Minimum length for a value to be treated as a bare high-entropy token.
const BARE_TOKEN_MIN_LEN: usize = 32;

/// Scan one string value from the manifest. `field` is the manifest key path
/// (e.g. `project.name`) and is the only detail that appears in the error.
pub fn check_value(field: &str, value: &str) -> Result<(), InnoframeworkError> {
    for (prefix, rule) in SECRET_PREFIXES {
        // PEM headers can appear inside a multi-line value, so they match
        // anywhere. Other credential prefixes must start the value.
        let matched = if prefix.starts_with("-----") {
            value.contains(prefix)
        } else {
            value.starts_with(prefix)
        };
        if matched {
            return Err(reject(field, rule));
        }
    }

    // A long bare token with no spaces, mixing letters and digits, is treated
    // as a probable secret. Paths, URLs with spaces, and prose are not.
    let looks_bare_token = value.len() >= BARE_TOKEN_MIN_LEN
        && !value.contains(char::is_whitespace)
        && value.chars().any(|c| c.is_ascii_alphabetic())
        && value.chars().any(|c| c.is_ascii_digit())
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '+' | '/' | '='));
    if looks_bare_token {
        return Err(reject(field, "high-entropy token"));
    }

    Ok(())
}

fn reject(field: &str, rule: &str) -> InnoframeworkError {
    // Never include the value. Only the field and the rule name are reported.
    InnoframeworkError::new(
        SECRET_IN_MANIFEST,
        format!("manifest field `{field}` looks like a {rule}; secrets must not be stored in the manifest"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_values_pass() {
        for v in [
            "my-app",
            "fs.read",
            "https://example.com/api",
            "C:/projects/my app/data",
            "a plain sentence with spaces that is long enough to matter",
        ] {
            assert!(check_value("f", v).is_ok(), "value {v:?} should pass");
        }
    }

    #[test]
    fn known_credential_prefixes_are_rejected() {
        for v in [
            "AKIAIOSFODNN7EXAMPLE",
            "ghp_abcdefghijklmnopqrstuvwxyz0123456789",
            "sk_live_51H8abcdefghijkl",
            "xoxb-123-456-abc",
            "sk-ant-api03-abc",
            "-----BEGIN PRIVATE KEY-----",
        ] {
            let err = check_value("database.password", v).unwrap_err();
            assert_eq!(err.code(), SECRET_IN_MANIFEST, "value {v:?}");
        }
    }

    #[test]
    fn long_bare_token_is_rejected() {
        let v = "Zx9Kq2Lm7Rt4Yw8Pe3Nb6Vc1Ds5Fg0Hj"; // 32 chars, letters + digits, no spaces
        assert_eq!(
            check_value("api.key", v).unwrap_err().code(),
            SECRET_IN_MANIFEST
        );
    }

    #[test]
    fn long_word_without_digits_passes() {
        // A long identifier with no digits is not treated as a token.
        assert!(check_value("f", "abcdefghijklmnopqrstuvwxyzabcdefghij").is_ok());
    }

    #[test]
    fn error_never_contains_the_value() {
        let secret = "AKIAIOSFODNN7EXAMPLE";
        let err = check_value("aws.key", secret).unwrap_err();
        assert!(!err.user_message().contains(secret));
        assert!(!err.user_message().contains("AKIA"));
        assert!(err.user_message().contains("aws.key"));
    }
}
