//! Capability model (ARCHITECTURE_PLAN.md §2.3, TR-3.2).
//!
//! A platform API call is allowed only when the manifest declares a
//! `Capability` with a matching id and platform. Authorization is a pure
//! function: no I/O, no ambient authority (fail closed).

use crate::error::{ErrorCode, InnoframeworkError};
use crate::validation::{invalid, Validate};

/// Error code for a platform call that no declared capability permits.
pub const CAPABILITY_DENIED: ErrorCode = ErrorCode::new_const(1100);

/// Target platform a capability applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    Web,
    Desktop,
    Mobile,
}

/// Restriction on where a capability may act.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// The capability has no path or resource restriction.
    Unrestricted,
    /// Only the listed path prefixes are allowed.
    PathPrefixes(Vec<String>),
}

/// A declared capability, e.g. `fs.read`, `notify.send`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    id: String,
    platforms: Vec<Platform>,
    scope: Scope,
}

impl Capability {
    /// Create a capability. The id must be non-empty and contain only
    /// lowercase ASCII letters, digits, `.`, `_`, or `-`.
    pub fn new(
        id: impl Into<String>,
        platforms: Vec<Platform>,
        scope: Scope,
    ) -> Result<Self, InnoframeworkError> {
        let cap = Self {
            id: id.into(),
            platforms,
            scope,
        };
        cap.validate()?;
        Ok(cap)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn platforms(&self) -> &[Platform] {
        &self.platforms
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }
}

impl Validate for Capability {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        let id_ok = !self.id.is_empty()
            && self.id.bytes().all(|b| {
                b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-')
            });
        if !id_ok {
            return Err(invalid(format!("invalid capability id: {:?}", self.id)));
        }
        if self.platforms.is_empty() {
            return Err(invalid(format!(
                "capability {} declares no platform",
                self.id
            )));
        }
        Ok(())
    }
}

/// Decide whether `requested_id` on `platform` is permitted by `declared`.
///
/// Fail closed: returns `Err(CAPABILITY_DENIED)` unless a declared capability
/// matches both the id and the platform.
pub fn authorize(
    declared: &[Capability],
    requested_id: &str,
    platform: Platform,
) -> Result<(), InnoframeworkError> {
    let allowed = declared
        .iter()
        .any(|c| c.id == requested_id && c.platforms.contains(&platform));
    if allowed {
        Ok(())
    } else {
        Err(InnoframeworkError::new(
            CAPABILITY_DENIED,
            format!("capability {requested_id} is not declared for this platform"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fs_read_desktop() -> Capability {
        Capability::new("fs.read", vec![Platform::Desktop], Scope::Unrestricted).unwrap()
    }

    #[test]
    fn declared_capability_on_declared_platform_is_allowed() {
        assert!(authorize(&[fs_read_desktop()], "fs.read", Platform::Desktop).is_ok());
    }

    #[test]
    fn undeclared_capability_is_denied() {
        let err = authorize(&[fs_read_desktop()], "fs.write", Platform::Desktop).unwrap_err();
        assert_eq!(err.code(), CAPABILITY_DENIED);
    }

    #[test]
    fn declared_capability_on_other_platform_is_denied() {
        // Declared for desktop only; a web call must fail (TR-3.2).
        let err = authorize(&[fs_read_desktop()], "fs.read", Platform::Web).unwrap_err();
        assert_eq!(err.code(), CAPABILITY_DENIED);
    }

    #[test]
    fn empty_manifest_denies_everything() {
        assert!(authorize(&[], "fs.read", Platform::Desktop).is_err());
    }

    #[test]
    fn invalid_id_is_rejected() {
        for bad in ["", "FS.READ", "fs read", "fs/read", "../etc"] {
            assert!(
                Capability::new(bad, vec![Platform::Web], Scope::Unrestricted).is_err(),
                "id {bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn capability_without_platform_is_rejected() {
        assert!(Capability::new("fs.read", vec![], Scope::Unrestricted).is_err());
    }
}
