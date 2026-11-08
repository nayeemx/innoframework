//! Manifest TOML parser (`inno.toml`, ARCHITECTURE_PLAN.md §2.3).
//!
//! Parsing is two steps:
//! 1. Deserialize into private raw structs. `deny_unknown_fields` rejects typos.
//! 2. Convert raw values through the validated constructors. Every string value
//!    passes the secret boundary check first.
//!
//! The public types (`Manifest`, `Capability`) do not implement `Deserialize`,
//! so no other code path can construct them without validation.

use serde::Deserialize;

use crate::capability::{Capability, Platform, Scope};
use crate::error::{ErrorCode, InnoframeworkError};
use crate::manifest::{Manifest, TargetKind};
use crate::secret;
use crate::validation::Validate;

/// Error code for a manifest that is not valid TOML or has an unknown field.
pub const MANIFEST_PARSE_ERROR: ErrorCode = ErrorCode::new_const(1210);

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    project: RawProject,
    #[serde(default)]
    capabilities: Vec<RawCapability>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProject {
    name: String,
    version: String,
    targets: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCapability {
    id: String,
    platforms: Vec<String>,
    #[serde(default)]
    path_prefixes: Option<Vec<String>>,
}

/// Parse and validate an `inno.toml` document.
pub fn parse_manifest(source: &str) -> Result<Manifest, InnoframeworkError> {
    let raw: RawManifest = toml::from_str(source).map_err(|e| {
        // The TOML error text can echo a value from the file. Report only
        // the position, and keep the detail for internal logs.
        InnoframeworkError::new(
            MANIFEST_PARSE_ERROR,
            "manifest is not valid TOML or contains an unknown field",
        )
        .with_details(e.to_string())
    })?;

    scan_secrets(&raw)?;

    let targets = raw
        .project
        .targets
        .iter()
        .map(|t| parse_target(t))
        .collect::<Result<Vec<_>, _>>()?;

    let capabilities = raw
        .capabilities
        .iter()
        .map(parse_capability)
        .collect::<Result<Vec<_>, _>>()?;

    let manifest = Manifest::new(raw.project.name, raw.project.version, targets, capabilities)?;
    manifest.validate()?;
    Ok(manifest)
}

fn scan_secrets(raw: &RawManifest) -> Result<(), InnoframeworkError> {
    secret::check_value("project.name", &raw.project.name)?;
    secret::check_value("project.version", &raw.project.version)?;
    for (i, t) in raw.project.targets.iter().enumerate() {
        secret::check_value(&format!("project.targets[{i}]"), t)?;
    }
    for (i, cap) in raw.capabilities.iter().enumerate() {
        secret::check_value(&format!("capabilities[{i}].id"), &cap.id)?;
        for (j, p) in cap.platforms.iter().enumerate() {
            secret::check_value(&format!("capabilities[{i}].platforms[{j}]"), p)?;
        }
        if let Some(prefixes) = &cap.path_prefixes {
            for (j, p) in prefixes.iter().enumerate() {
                secret::check_value(&format!("capabilities[{i}].path_prefixes[{j}]"), p)?;
            }
        }
    }
    Ok(())
}

fn parse_target(s: &str) -> Result<TargetKind, InnoframeworkError> {
    match s {
        "web" => Ok(TargetKind::Web),
        "desktop" => Ok(TargetKind::Desktop),
        "mobile" => Ok(TargetKind::Mobile),
        other => Err(InnoframeworkError::new(
            MANIFEST_PARSE_ERROR,
            format!("unknown target `{other}`; expected web, desktop, or mobile"),
        )),
    }
}

fn parse_platform(s: &str) -> Result<Platform, InnoframeworkError> {
    match s {
        "web" => Ok(Platform::Web),
        "desktop" => Ok(Platform::Desktop),
        "mobile" => Ok(Platform::Mobile),
        other => Err(InnoframeworkError::new(
            MANIFEST_PARSE_ERROR,
            format!("unknown platform `{other}`; expected web, desktop, or mobile"),
        )),
    }
}

fn parse_capability(raw: &RawCapability) -> Result<Capability, InnoframeworkError> {
    let platforms = raw
        .platforms
        .iter()
        .map(|p| parse_platform(p))
        .collect::<Result<Vec<_>, _>>()?;
    let scope = match &raw.path_prefixes {
        None => Scope::Unrestricted,
        Some(prefixes) => Scope::PathPrefixes(prefixes.clone()),
    };
    Capability::new(raw.id.clone(), platforms, scope)
}
