//! Golden and negative tests for manifest TOML parsing.
//!
//! Fixtures live in `tests/fixtures/`. A valid `.toml` has a matching
//! `.summary` text file; an invalid `.toml` must return its expected error
//! code.

use innoframework_core::manifest::MANIFEST_ERROR;
use innoframework_core::manifest_toml::{parse_manifest, MANIFEST_PARSE_ERROR};
use innoframework_core::secret::SECRET_IN_MANIFEST;

fn load(name: &str) -> String {
    std::fs::read_to_string(std::path::PathBuf::from_iter(["tests", "fixtures", name])).unwrap()
}

#[test]
fn valid_basic_parses() {
    let m = parse_manifest(&load("valid_basic.toml")).unwrap();
    assert_eq!(m.name(), "my-app");
    assert_eq!(m.version(), "1.2.3");
    assert_eq!(m.targets().len(), 2);
    assert_eq!(m.capabilities().len(), 2);
}

#[test]
fn valid_basic_matches_summary() {
    let source = load("valid_basic.toml");
    let m = parse_manifest(&source).unwrap();
    let summary = format!(
        "name={}\nversion={}\ntargets={}\n{}",
        m.name(),
        m.version(),
        m.targets()
            .iter()
            .map(|t| match t {
                innoframework_core::manifest::TargetKind::Web => "web",
                innoframework_core::manifest::TargetKind::Desktop => "desktop",
                innoframework_core::manifest::TargetKind::Mobile => "mobile",
            })
            .collect::<Vec<_>>()
            .join(","),
        m.capabilities()
            .iter()
            .map(|cap| {
                let platforms = cap
                    .platforms()
                    .iter()
                    .map(|p| match p {
                        innoframework_core::capability::Platform::Web => "web",
                        innoframework_core::capability::Platform::Desktop => "desktop",
                        innoframework_core::capability::Platform::Mobile => "mobile",
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                match cap.scope() {
                    innoframework_core::capability::Scope::Unrestricted => {
                        format!(
                            "capability={} platforms={} scope=unrestricted",
                            cap.id(),
                            platforms
                        )
                    }
                    innoframework_core::capability::Scope::PathPrefixes(prefixes) => format!(
                        "capability={} platforms={} scope=path_prefixes:{}",
                        cap.id(),
                        platforms,
                        prefixes.join(",")
                    ),
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
    let expected = load("valid_basic.summary");
    assert_eq!(summary, expected.trim());
}

#[test]
fn invalid_secret_in_manifest_rejected() {
    let err = parse_manifest(&load("invalid_secret_in_manifest.toml")).unwrap_err();
    assert_eq!(err.code(), SECRET_IN_MANIFEST);
}

#[test]
fn invalid_unknown_field_rejected() {
    let err = parse_manifest(&load("invalid_unknown_field.toml")).unwrap_err();
    assert_eq!(err.code(), MANIFEST_PARSE_ERROR);
}

#[test]
fn invalid_unknown_capability_field_rejected() {
    let err = parse_manifest(&load("invalid_unknown_capability_field.toml")).unwrap_err();
    assert_eq!(err.code(), MANIFEST_PARSE_ERROR);
}

#[test]
fn invalid_capability_target_rejected() {
    let toml = r#"[project]
name = "app"
version = "1.0.0"
targets = ["web"]

[[capabilities]]
id = "fs.read"
platforms = ["dssktop"]
"#;
    let err = parse_manifest(toml).unwrap_err();
    assert_eq!(err.code(), MANIFEST_PARSE_ERROR);
}

#[test]
fn invalid_manifest_target_rejected() {
    let toml = r#"[project]
name = "app"
version = "1.0.0"
targets = ["cloud"]
"#;
    let err = parse_manifest(toml).unwrap_err();
    assert_eq!(err.code(), MANIFEST_PARSE_ERROR);
}

#[test]
fn invalid_name_rejected() {
    let toml = r#"[project]
name = "My App"
version = "1.0.0"
targets = ["web"]
"#;
    let err = parse_manifest(toml).unwrap_err();
    assert_eq!(err.code(), MANIFEST_ERROR);
}

#[test]
fn empty_targets_rejected() {
    let toml = r#"[project]
name = "app"
version = "1.0.0"
targets = []
"#;
    let err = parse_manifest(toml).unwrap_err();
    assert_eq!(err.code(), MANIFEST_ERROR);
}
