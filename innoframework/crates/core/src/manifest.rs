//! Manifest model (ARCHITECTURE_PLAN.md §2.3, inno.toml).
//!
//! The manifest is the single source of truth for a generated project:
//! name, version, targets, capabilities, and config sections. Unknown
//! capability ids are rejected. Secrets must never appear as values
//! (enforced by the secret boundary check, see P1-5).

use crate::capability::{Capability, Platform};
use crate::error::{ErrorCode, InnoframeworkError};
use crate::validation::Validate;

/// Error code for manifest validation failures.
pub const MANIFEST_ERROR: ErrorCode = ErrorCode::new_const(1200);

/// Target kind for a build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetKind {
    Web,
    Desktop,
    Mobile,
}

impl TargetKind {
    pub fn as_platform(self) -> Platform {
        match self {
            TargetKind::Web => Platform::Web,
            TargetKind::Desktop => Platform::Desktop,
            TargetKind::Mobile => Platform::Mobile,
        }
    }
}

/// Parsed project manifest.
#[derive(Debug, Clone)]
pub struct Manifest {
    name: String,
    version: String,
    targets: Vec<TargetKind>,
    capabilities: Vec<Capability>,
}

impl Manifest {
    /// Create a manifest. All invariants are checked.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        targets: Vec<TargetKind>,
        capabilities: Vec<Capability>,
    ) -> Result<Self, InnoframeworkError> {
        let m = Self {
            name: name.into(),
            version: version.into(),
            targets,
            capabilities,
        };
        m.validate()?;
        Ok(m)
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn targets(&self) -> &[TargetKind] {
        &self.targets
    }

    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }
}

impl Validate for Manifest {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        // Name: non-empty, lowercase, ASCII letters/digits/dash only.
        let name_ok = !self.name.is_empty()
            && self
                .name
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
        if !name_ok {
            return Err(InnoframeworkError::new(
                MANIFEST_ERROR,
                format!("invalid manifest name: {:?}", self.name),
            ));
        }

        // Version: must look like semver (X.Y.Z) without pre-release for now.
        let parts: Vec<&str> = self.version.split('.').collect();
        let version_ok = parts.len() == 3
            && parts
                .iter()
                .all(|p| p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty());
        if !version_ok {
            return Err(InnoframeworkError::new(
                MANIFEST_ERROR,
                format!("invalid version (expected X.Y.Z): {:?}", self.version),
            ));
        }

        // Targets: at least one.
        if self.targets.is_empty() {
            return Err(InnoframeworkError::new(
                MANIFEST_ERROR,
                "manifest must declare at least one target",
            ));
        }

        // Capabilities: validate each one (already validated at construction,
        // but check again since the manifest could be deserialized).
        for cap in &self.capabilities {
            cap.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Scope;

    fn fs_read_web() -> Capability {
        Capability::new("fs.read", vec![Platform::Web], Scope::Unrestricted).unwrap()
    }

    #[test]
    fn valid_manifest_passes() {
        let m = Manifest::new(
            "my-app",
            "1.0.0",
            vec![TargetKind::Web],
            vec![fs_read_web()],
        )
        .unwrap();
        assert_eq!(m.name(), "my-app");
        assert_eq!(m.version(), "1.0.0");
        assert_eq!(m.targets().len(), 1);
        assert_eq!(m.capabilities().len(), 1);
    }

    #[test]
    fn empty_name_rejected() {
        assert!(Manifest::new("", "1.0.0", vec![TargetKind::Web], vec![]).is_err());
    }

    #[test]
    fn uppercase_name_rejected() {
        assert!(Manifest::new("MyApp", "1.0.0", vec![TargetKind::Web], vec![]).is_err());
    }

    #[test]
    fn bad_version_rejected() {
        for bad in ["1.0", "1.0.0-beta", "v1.0.0", "", "1.0.x"] {
            assert!(
                Manifest::new("app", bad, vec![TargetKind::Web], vec![]).is_err(),
                "version {bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn no_targets_rejected() {
        assert!(Manifest::new("app", "1.0.0", vec![], vec![]).is_err());
    }

    #[test]
    fn multiple_targets_allowed() {
        let m = Manifest::new(
            "app",
            "0.1.0",
            vec![TargetKind::Web, TargetKind::Desktop, TargetKind::Mobile],
            vec![],
        )
        .unwrap();
        assert_eq!(m.targets().len(), 3);
    }
}
