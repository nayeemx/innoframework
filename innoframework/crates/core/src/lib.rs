//! innoframework-core: error model, sessions, roles, and the fail-closed auth guard.
//!
//! Token format: `base64url(payload_json).base64url(hmac_sha256(payload_bytes))`.
//! Verification is constant-time and rejects expired tokens. Every rejection
//! returns the same `AuthError::Invalid` variant on the public path so callers
//! cannot distinguish a forged token from a malformed one (SEC-RPC-4).

pub mod agent;
pub mod build_target;
pub mod capability;
pub mod error;
pub mod manifest;
pub mod manifest_toml;
pub mod secret;
pub mod tool;
pub mod validation;

pub use error::{ErrorCode, InnoframeworkError};
pub use manifest::Manifest;
pub use manifest_toml::parse_manifest;
pub use validation::Validate;

use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Minimum accepted secret length in bytes. Shorter keys are refused at startup.
pub const MIN_SECRET_LEN: usize = 32;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuthError {
    /// Any reason a token is not accepted. Intentionally undifferentiated.
    #[error("unauthenticated")]
    Invalid,
    /// The token is valid but the role is not sufficient. Maps to 403.
    #[error("forbidden")]
    Forbidden,
    /// Secret is missing or too short. Configuration error, not a request error.
    #[error("secret must be at least {MIN_SECRET_LEN} bytes")]
    WeakSecret,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub role: Role,
    /// Unix seconds.
    pub issued_at: u64,
    /// Unix seconds. Must be strictly greater than `issued_at`.
    pub expires_at: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Admin,
}

/// Signing key wrapper. Refuses short keys so weak configuration fails at startup.
#[derive(Clone)]
pub struct SessionKey(Vec<u8>);

impl SessionKey {
    pub fn new(secret: &[u8]) -> Result<Self, AuthError> {
        if secret.len() < MIN_SECRET_LEN {
            return Err(AuthError::WeakSecret);
        }
        Ok(Self(secret.to_vec()))
    }

    fn mac(&self) -> HmacSha256 {
        // HMAC accepts keys of any length, so this cannot fail.
        HmacSha256::new_from_slice(&self.0).expect("hmac accepts any key length")
    }
}

impl std::fmt::Debug for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print key material.
        f.write_str("SessionKey(<redacted>)")
    }
}

/// Create a signed token for a session.
pub fn issue(key: &SessionKey, session: &Session) -> String {
    let payload = serde_json::to_vec(session).expect("session serializes");
    let mut mac = key.mac();
    mac.update(&payload);
    let sig = mac.finalize().into_bytes();
    format!("{}.{}", B64.encode(&payload), B64.encode(sig))
}

/// Verify a token and return the session. `now` is injected for testability.
pub fn verify(key: &SessionKey, token: &str, now: u64) -> Result<Session, AuthError> {
    let (p64, s64) = token.split_once('.').ok_or(AuthError::Invalid)?;
    let payload = B64.decode(p64).map_err(|_| AuthError::Invalid)?;
    let sig = B64.decode(s64).map_err(|_| AuthError::Invalid)?;

    let mut mac = key.mac();
    mac.update(&payload);
    let expected = mac.finalize().into_bytes();
    // Constant-time comparison of the signature.
    if expected.len() != sig.len() || expected.ct_eq(&sig).unwrap_u8() != 1 {
        return Err(AuthError::Invalid);
    }

    let session: Session = serde_json::from_slice(&payload).map_err(|_| AuthError::Invalid)?;
    if session.expires_at <= session.issued_at || now >= session.expires_at {
        return Err(AuthError::Invalid);
    }
    Ok(session)
}

/// Fail-closed role check. Returns `Forbidden` when the role is insufficient.
pub fn require_role(session: &Session, required: Role) -> Result<(), AuthError> {
    if session.role >= required {
        Ok(())
    } else {
        Err(AuthError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"0123456789abcdef0123456789abcdef";

    fn key() -> SessionKey {
        SessionKey::new(SECRET).unwrap()
    }

    fn session(role: Role) -> Session {
        Session {
            user_id: "u1".into(),
            role,
            issued_at: 1000,
            expires_at: 2000,
        }
    }

    #[test]
    fn valid_token_verifies() {
        let t = issue(&key(), &session(Role::User));
        assert_eq!(verify(&key(), &t, 1500).unwrap(), session(Role::User));
    }

    #[test]
    fn expired_token_is_rejected() {
        let t = issue(&key(), &session(Role::User));
        assert_eq!(verify(&key(), &t, 2000), Err(AuthError::Invalid));
    }

    #[test]
    fn tampered_payload_is_rejected() {
        let t = issue(&key(), &session(Role::User));
        let (_, sig) = t.split_once('.').unwrap();
        let forged = B64.encode(serde_json::to_vec(&session(Role::Admin)).unwrap());
        let tampered = format!("{forged}.{sig}");
        assert_eq!(verify(&key(), &tampered, 1500), Err(AuthError::Invalid));
    }

    #[test]
    fn wrong_key_is_rejected() {
        let t = issue(&key(), &session(Role::User));
        let other = SessionKey::new(b"fedcba9876543210fedcba9876543210").unwrap();
        assert_eq!(verify(&other, &t, 1500), Err(AuthError::Invalid));
    }

    #[test]
    fn malformed_tokens_are_rejected() {
        for bad in ["", "nodot", "a.b", ".", "!!!.???"] {
            assert_eq!(
                verify(&key(), bad, 1500),
                Err(AuthError::Invalid),
                "input {bad:?}"
            );
        }
    }

    #[test]
    fn expires_before_issue_is_rejected() {
        let bad = Session {
            user_id: "u".into(),
            role: Role::User,
            issued_at: 5,
            expires_at: 5,
        };
        let t = issue(&key(), &bad);
        assert_eq!(verify(&key(), &t, 1), Err(AuthError::Invalid));
    }

    #[test]
    fn short_secret_is_refused() {
        assert_eq!(
            SessionKey::new(b"short").unwrap_err(),
            AuthError::WeakSecret
        );
    }

    #[test]
    fn user_cannot_access_admin_route() {
        assert_eq!(
            require_role(&session(Role::User), Role::Admin),
            Err(AuthError::Forbidden)
        );
        assert!(require_role(&session(Role::Admin), Role::Admin).is_ok());
    }

    #[test]
    fn debug_does_not_leak_key() {
        assert!(!format!("{:?}", key()).contains("0123"));
    }
}
