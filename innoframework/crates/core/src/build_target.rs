//! Build target model (ARCHITECTURE_PLAN.md §2.3).
//!
//! Mobile targets require a signing identity. If it is missing, the build
//! fails with a typed error and a fix message (TR-3.2).

use crate::error::{ErrorCode, InnoframeworkError};
use crate::validation::{invalid, Validate};

/// Error code for build target validation failures.
pub const BUILD_TARGET_ERROR: ErrorCode = ErrorCode::new_const(1500);

/// Bundle output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundleFormat {
    Wasm,
    Msi,
    Dmg,
    Apk,
    Aab,
    Ipa,
}

/// A target the build pipeline can produce.
#[derive(Debug, Clone)]
pub struct BuildTarget {
    kind: TargetKind,
    triple: String,
    bundle_format: BundleFormat,
    /// Signing identity (required for mobile, optional otherwise).
    signing_identity: Option<String>,
}

/// Target kind (re-exported from manifest for convenience).
pub use crate::manifest::TargetKind;

impl BuildTarget {
    pub fn new(
        kind: TargetKind,
        triple: impl Into<String>,
        bundle_format: BundleFormat,
        signing_identity: Option<String>,
    ) -> Result<Self, InnoframeworkError> {
        let target = Self {
            kind,
            triple: triple.into(),
            bundle_format,
            signing_identity,
        };
        target.validate()?;
        Ok(target)
    }

    pub fn kind(&self) -> TargetKind {
        self.kind
    }

    pub fn triple(&self) -> &str {
        &self.triple
    }

    pub fn bundle_format(&self) -> BundleFormat {
        self.bundle_format
    }

    pub fn signing_identity(&self) -> Option<&str> {
        self.signing_identity.as_deref()
    }
}

impl Validate for BuildTarget {
    fn validate(&self) -> Result<(), InnoframeworkError> {
        if self.triple.is_empty() {
            return Err(invalid("build target triple cannot be empty"));
        }

        // Mobile requires a signing identity (ADR-13: building is in scope,
        // but the build must declare one, even if it is a self-signed test
        // identity).
        if self.kind == TargetKind::Mobile && self.signing_identity.is_none() {
            return Err(InnoframeworkError::new(
                BUILD_TARGET_ERROR,
                "mobile build requires a signing identity. Set one with: inno config set signing-identity <id>",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_target_without_signing_is_ok() {
        let t = BuildTarget::new(
            TargetKind::Web,
            "wasm32-unknown-unknown",
            BundleFormat::Wasm,
            None,
        )
        .unwrap();
        assert_eq!(t.kind(), TargetKind::Web);
        assert_eq!(t.triple(), "wasm32-unknown-unknown");
    }

    #[test]
    fn desktop_target_without_signing_is_ok() {
        let t = BuildTarget::new(
            TargetKind::Desktop,
            "x86_64-pc-windows-msvc",
            BundleFormat::Msi,
            None,
        )
        .unwrap();
        assert!(t.signing_identity().is_none());
    }

    #[test]
    fn mobile_target_without_signing_is_rejected() {
        let err = BuildTarget::new(
            TargetKind::Mobile,
            "aarch64-linux-android",
            BundleFormat::Apk,
            None,
        )
        .unwrap_err();
        assert_eq!(err.code(), BUILD_TARGET_ERROR);
        assert!(err.user_message().contains("signing identity"));
    }

    #[test]
    fn mobile_target_with_signing_is_ok() {
        let t = BuildTarget::new(
            TargetKind::Mobile,
            "aarch64-linux-android",
            BundleFormat::Apk,
            Some("test-identity".to_string()),
        )
        .unwrap();
        assert_eq!(t.signing_identity(), Some("test-identity"));
    }

    #[test]
    fn empty_triple_rejected() {
        assert!(BuildTarget::new(TargetKind::Web, "", BundleFormat::Wasm, None).is_err());
    }
}
