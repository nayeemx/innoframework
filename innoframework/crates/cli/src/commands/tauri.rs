//! Tauri 2 build integration (P5/P9, ADR-02).
//!
//! Generates Tauri project files (`tauri.conf.json`, capability JSONs) from
//! the project manifest, then runs `cargo tauri build` to produce the
//! desktop or mobile bundle. Capabilities declared in `inno.toml` are mapped
//! to Tauri permissions and allowlisted commands — fail closed: undeclared
//! native APIs are not exposed (TR-3.2).
//!
//! Mobile targets require a signing identity (ADR-09 deferred: unsigned
//! bundles only until a certificate/account exists).

use crate::errors::InnoframeworkCliError;
use innoframework_core::capability::{Capability, Platform, Scope};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;

pub const TAURI_DIR: &str = "src-tauri";
pub const CAPABILITIES_DIR: &str = "capabilities";

/// Mobile build target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileTarget {
    Android,
    IOS,
}

impl MobileTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            MobileTarget::Android => "android",
            MobileTarget::IOS => "ios",
        }
    }

    /// Rust target triples for this mobile platform.
    pub fn triples(&self) -> &'static [&'static str] {
        match self {
            MobileTarget::Android => &[
                "aarch64-linux-android",
                "armv7-linux-androideabi",
                "x86_64-linux-android",
            ],
            MobileTarget::IOS => &["aarch64-apple-ios", "x86_64-apple-ios"],
        }
    }

    /// Tauri bundle target string for `cargo tauri build`.
    pub fn bundle_target(&self) -> &'static str {
        match self {
            MobileTarget::Android => "android-aab",
            MobileTarget::IOS => "ios-ipa",
        }
    }
}

/// Map an Innoframework capability id to Tauri permissions.
fn capability_permissions(cap: &Capability) -> Vec<String> {
    let mut perms = Vec::new();
    match cap.id() {
        "fs.read" => {
            perms.extend([
                "fs:read-file".to_string(),
                "fs:read-dir".to_string(),
                "fs:read-text-file".to_string(),
                "fs:copy-file".to_string(),
            ]);
        }
        "fs.write" => {
            perms.extend([
                "fs:write-file".to_string(),
                "fs:create-dir".to_string(),
                "fs:delete-file".to_string(),
                "fs:rename-file".to_string(),
            ]);
        }
        "notify.send" => {
            perms.push("notification:show".to_string());
        }
        "keychain.get" => {
            perms.push("keychain:get".to_string());
        }
        "keychain.set" => {
            perms.push("keychain:set".to_string());
        }
        "dialog.open" => {
            perms.push("dialog:open".to_string());
        }
        "dialog.save" => {
            perms.push("dialog:save".to_string());
        }
        "shell.execute" => {
            perms.push("shell:execute".to_string());
        }
        _ => {
            // Unknown capability ids are not mapped — fail closed.
        }
    }
    perms
}

/// Map a capability id to a Tauri capability identifier.
fn capability_identifier(cap_id: &str) -> String {
    cap_id.replace('.', "-")
}

/// Filter capabilities for a given platform.
fn filter_by_platform(capabilities: &[Capability], platform: Platform) -> Vec<&Capability> {
    capabilities
        .iter()
        .filter(|c| c.platforms().contains(&platform))
        .collect()
}

/// Generate `tauri.conf.json` for the desktop target.
pub fn generate_tauri_conf_desktop(
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
) -> Value {
    let desktop_caps = filter_by_platform(capabilities, Platform::Desktop);
    build_tauri_conf(
        project_name,
        version,
        &desktop_caps,
        csp,
        true, // is_desktop
    )
}

/// Generate `tauri.conf.json` for the mobile target.
pub fn generate_tauri_conf_mobile(
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
    _mobile_target: MobileTarget,
) -> Value {
    let mobile_caps = filter_by_platform(capabilities, Platform::Mobile);
    build_tauri_conf(
        project_name,
        version,
        &mobile_caps,
        csp,
        false, // is_desktop
    )
}

/// Shared tauri.conf.json builder.
fn build_tauri_conf(
    project_name: &str,
    version: &str,
    capabilities: &[&Capability],
    csp: &str,
    is_desktop: bool,
) -> Value {
    let mut allowlist = Map::new();

    let mut notification = Map::new();
    let mut keychain = Map::new();
    let mut dialog = Map::new();
    let mut shell = Map::new();

    let mut tauri_capabilities = Vec::new();

    for cap in capabilities {
        let id = capability_identifier(cap.id());
        tauri_capabilities.push(id.clone());

        for perm in capability_permissions(cap) {
            let (section, key) = perm.split_once(':').unwrap_or((perm.as_str(), ""));
            match section {
                "notification" => {
                    notification.insert(key.to_string(), Value::Bool(true));
                }
                "keychain" => {
                    keychain.insert(key.to_string(), Value::Bool(true));
                }
                "dialog" => {
                    dialog.insert(key.to_string(), Value::Bool(true));
                }
                "shell" => {
                    shell.insert(key.to_string(), Value::Bool(true));
                }
                _ => {}
            }
        }
    }

    // Build fs allowlist from actual permissions.
    let mut fs_allowlist = Map::new();
    for cap in capabilities {
        match cap.id() {
            "fs.read" => {
                fs_allowlist.insert("readFile".to_string(), Value::Bool(true));
                fs_allowlist.insert("readDir".to_string(), Value::Bool(true));
                fs_allowlist.insert("readTextFile".to_string(), Value::Bool(true));
                fs_allowlist.insert("copyFile".to_string(), Value::Bool(true));
            }
            "fs.write" => {
                fs_allowlist.insert("writeFile".to_string(), Value::Bool(true));
                fs_allowlist.insert("createDir".to_string(), Value::Bool(true));
                fs_allowlist.insert("deleteFile".to_string(), Value::Bool(true));
                fs_allowlist.insert("renameFile".to_string(), Value::Bool(true));
                // Add scope from path prefixes if declared.
                if let Scope::PathPrefixes(prefixes) = &cap.scope() {
                    if !prefixes.is_empty() {
                        fs_allowlist.insert(
                            "scope".to_string(),
                            Value::Array(
                                prefixes.iter().map(|p| Value::String(p.clone())).collect(),
                            ),
                        );
                    }
                }
            }
            _ => {}
        }
    }
    if !fs_allowlist.is_empty() {
        allowlist.insert("fs".to_string(), Value::Object(fs_allowlist));
    }
    if !notification.is_empty() {
        allowlist.insert("notification".to_string(), Value::Object(notification));
    }
    if !keychain.is_empty() {
        allowlist.insert("keychain".to_string(), Value::Object(keychain));
    }
    if !dialog.is_empty() {
        allowlist.insert("dialog".to_string(), Value::Object(dialog));
    }
    if !shell.is_empty() {
        allowlist.insert("shell".to_string(), Value::Object(shell));
    }

    let mut app = Map::new();
    let mut security = Map::new();
    security.insert("csp".to_string(), Value::String(csp.to_string()));
    app.insert("security".to_string(), Value::Object(security));

    if is_desktop {
        let mut windows_item = Map::new();
        windows_item.insert("title".to_string(), Value::String(project_name.to_string()));
        windows_item.insert("width".to_string(), Value::Number(800.into()));
        windows_item.insert("height".to_string(), Value::Number(600.into()));
        windows_item.insert("resizable".to_string(), Value::Bool(true));
        app.insert(
            "windows".to_string(),
            Value::Array(vec![Value::Object(windows_item)]),
        );
    }

    // Mobile icons.
    let icon_list = if is_desktop {
        vec![
            Value::String("icons/32x32.png".to_string()),
            Value::String("icons/128x128.png".to_string()),
            Value::String("icons/128x128@2x.png".to_string()),
            Value::String("icons/icon.icns".to_string()),
            Value::String("icons/icon.ico".to_string()),
        ]
    } else {
        vec![
            Value::String("icons/icon.png".to_string()),
            Value::String("icons/icon@2x.png".to_string()),
            Value::String("icons/icon-adaptive.png".to_string()),
            Value::String("icons/icon-round.png".to_string()),
        ]
    };

    json!({
        "$schema": "../gen/schemas/desktop-schema.json",
        "productName": project_name,
        "version": version,
        "identifier": format!("com.innoframework.{}", project_name),
        "build": {
            "frontendDist": format!("../target/dx/{}/release/web/public", project_name),
            "devUrl": "http://localhost:1420",
            "beforeDevCommand": format!("inno dev --port 1420"),
            "beforeBuildCommand": "inno build web".to_string(),
        },
        "app": app,
        "tauri": {
            "allowlist": allowlist,
            "capabilities": tauri_capabilities,
            "bundle": {
                "active": true,
                "targets": if is_desktop { "all" } else { "mobile" },
                "identifier": format!("com.innoframework.{}", project_name),
                "icon": icon_list,
            },
        },
    })
}

/// Generate a Tauri capability JSON file for a single capability.
pub fn generate_capability_json(cap: &Capability) -> Value {
    json!({
        "$schema": "../gen/schemas/desktop-schema.json",
        "identifier": capability_identifier(cap.id()),
        "description": format!("Innoframework capability: {}", cap.id()),
        "capabilities": capability_permissions(cap),
    })
}

/// Write Tauri project files for the desktop target to `src-tauri/`.
pub fn write_tauri_project_desktop(
    root: &Path,
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
) -> Result<(), InnoframeworkCliError> {
    write_tauri_project_impl(root, project_name, version, capabilities, csp, true, None)
}

/// Write Tauri project files for the mobile target to `src-tauri/`.
pub fn write_tauri_project_mobile(
    root: &Path,
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
    mobile_target: MobileTarget,
) -> Result<(), InnoframeworkCliError> {
    write_tauri_project_impl(
        root,
        project_name,
        version,
        capabilities,
        csp,
        false,
        Some(mobile_target),
    )?;
    // Mobile-specific: emit a target hint file.
    let mobile_dir = root.join(TAURI_DIR).join("mobile");
    fs::create_dir_all(&mobile_dir).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot create src-tauri/mobile directory: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;
    let hint = format!("target={}\n", mobile_target.as_str());
    fs::write(mobile_dir.join("target.txt"), hint).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot write mobile target hint: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;
    Ok(())
}

/// Shared implementation for writing Tauri project files.
fn write_tauri_project_impl(
    root: &Path,
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
    is_desktop: bool,
    mobile_target: Option<MobileTarget>,
) -> Result<(), InnoframeworkCliError> {
    let tauri_dir = root.join(TAURI_DIR);
    let caps_dir = tauri_dir.join(CAPABILITIES_DIR);

    fs::create_dir_all(&tauri_dir).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot create src-tauri directory: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;
    fs::create_dir_all(&caps_dir).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot create src-tauri/capabilities directory: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Write tauri.conf.json.
    let conf = if is_desktop {
        generate_tauri_conf_desktop(project_name, version, capabilities, csp)
    } else {
        generate_tauri_conf_mobile(
            project_name,
            version,
            capabilities,
            csp,
            mobile_target.unwrap_or(MobileTarget::Android),
        )
    };
    let conf_json = serde_json::to_string_pretty(&conf).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot serialize tauri.conf.json: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;
    fs::write(tauri_dir.join("tauri.conf.json"), conf_json).map_err(|e| {
        InnoframeworkCliError::new(
            crate::errors::CliErrorCode::TemplateMissing,
            format!("cannot write tauri.conf.json: {e}"),
            "check disk space and permissions",
            None,
        )
    })?;

    // Write capability JSON files for the relevant platform.
    let platform = if is_desktop {
        Platform::Desktop
    } else {
        Platform::Mobile
    };
    for cap in capabilities {
        if !cap.platforms().contains(&platform) {
            continue;
        }
        let cap_json = generate_capability_json(cap);
        let cap_json_str = serde_json::to_string_pretty(&cap_json).map_err(|e| {
            InnoframeworkCliError::new(
                crate::errors::CliErrorCode::TemplateMissing,
                format!("cannot serialize capability {}: {e}", cap.id()),
                "check disk space and permissions",
                None,
            )
        })?;
        let cap_filename = format!("{}.json", capability_identifier(cap.id()));
        fs::write(caps_dir.join(&cap_filename), cap_json_str).map_err(|e| {
            InnoframeworkCliError::new(
                crate::errors::CliErrorCode::TemplateMissing,
                format!("cannot write capability file {cap_filename}: {e}"),
                "check disk space and permissions",
                None,
            )
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use innoframework_core::capability::Scope;

    fn fs_read_desktop() -> Capability {
        Capability::new("fs.read", vec![Platform::Desktop], Scope::Unrestricted).unwrap()
    }

    fn fs_write_scoped() -> Capability {
        Capability::new(
            "fs.write",
            vec![Platform::Desktop],
            Scope::PathPrefixes(vec!["data/".to_string(), "logs/".to_string()]),
        )
        .unwrap()
    }

    fn notify_send_desktop() -> Capability {
        Capability::new("notify.send", vec![Platform::Desktop], Scope::Unrestricted).unwrap()
    }

    fn fs_read_mobile() -> Capability {
        Capability::new("fs.read", vec![Platform::Mobile], Scope::Unrestricted).unwrap()
    }

    fn notify_send_mobile() -> Capability {
        Capability::new("notify.send", vec![Platform::Mobile], Scope::Unrestricted).unwrap()
    }

    #[test]
    fn generate_tauri_conf_includes_capabilities() {
        let caps = vec![fs_read_desktop(), notify_send_desktop()];
        let conf = generate_tauri_conf_desktop("my-app", "0.1.0", &caps, "default-src 'self'");
        let conf_str = serde_json::to_string(&conf).unwrap();
        assert!(conf_str.contains("com.innoframework.my-app"));
        assert!(conf_str.contains("fs-read"));
        assert!(conf_str.contains("notify-send"));
        assert!(conf_str.contains("readFile"));
        assert!(conf_str.contains("show"));
        assert!(!conf_str.contains("writeFile")); // fs.write not declared
    }

    #[test]
    fn generate_tauri_conf_excludes_other_platforms() {
        let web_cap = Capability::new("fs.read", vec![Platform::Web], Scope::Unrestricted).unwrap();
        let caps = vec![web_cap];
        let conf = generate_tauri_conf_desktop("my-app", "0.1.0", &caps, "default-src 'self'");
        let conf_str = serde_json::to_string(&conf).unwrap();
        assert!(!conf_str.contains("fs-read")); // web-only capability excluded
        let conf_val: Value = serde_json::from_str(&conf_str).unwrap();
        assert!(conf_val
            .get("tauri")
            .and_then(|t| t.get("capabilities"))
            .and_then(|c| c.as_array())
            .map_or(true, |a| a.is_empty()));
    }

    #[test]
    fn generate_tauri_conf_includes_scope() {
        let caps = vec![fs_write_scoped()];
        let conf = generate_tauri_conf_desktop("my-app", "0.1.0", &caps, "default-src 'self'");
        let conf_str = serde_json::to_string(&conf).unwrap();
        assert!(conf_str.contains("data/"));
        assert!(conf_str.contains("logs/"));
    }

    #[test]
    fn generate_tauri_conf_mobile_includes_mobile_caps() {
        let caps = vec![fs_read_mobile(), notify_send_mobile()];
        let conf = generate_tauri_conf_mobile(
            "my-app",
            "0.1.0",
            &caps,
            "default-src 'self'",
            MobileTarget::Android,
        );
        let conf_str = serde_json::to_string(&conf).unwrap();
        assert!(conf_str.contains("fs-read"));
        assert!(conf_str.contains("notify-send"));
        assert!(conf_str.contains("readFile"));
        assert!(conf_str.contains("show"));
        // Desktop-only things should not appear.
        assert!(!conf_str.contains("\"windows\""));
        assert!(!conf_str.contains("icon.icns"));
        assert!(!conf_str.contains("icon.ico"));
    }

    #[test]
    fn generate_tauri_conf_mobile_has_mobile_targets() {
        let caps: Vec<Capability> = vec![];
        let conf = generate_tauri_conf_mobile(
            "my-app",
            "0.1.0",
            &caps,
            "default-src 'self'",
            MobileTarget::Android,
        );
        let conf_val: Value = serde_json::to_value(&conf).unwrap();
        let targets = conf_val
            .get("tauri")
            .and_then(|t| t.get("bundle"))
            .and_then(|b| b.get("targets"))
            .and_then(|b| b.as_str())
            .unwrap();
        assert_eq!(targets, "mobile");
    }

    #[test]
    fn generate_tauri_conf_mobile_android_triples() {
        assert!(MobileTarget::Android
            .triples()
            .contains(&"aarch64-linux-android"));
        assert!(MobileTarget::Android
            .triples()
            .contains(&"armv7-linux-androideabi"));
        assert_eq!(MobileTarget::Android.bundle_target(), "android-aab");
    }

    #[test]
    fn generate_tauri_conf_mobile_ios_triples() {
        assert!(MobileTarget::IOS.triples().contains(&"aarch64-apple-ios"));
        assert!(MobileTarget::IOS.triples().contains(&"x86_64-apple-ios"));
        assert_eq!(MobileTarget::IOS.bundle_target(), "ios-ipa");
    }

    #[test]
    fn generate_capability_json_has_permissions() {
        let cap = fs_read_desktop();
        let cap_json = generate_capability_json(&cap);
        let cap_str = serde_json::to_string(&cap_json).unwrap();
        assert_eq!(
            cap_json.get("identifier"),
            Some(&Value::String("fs-read".to_string()))
        );
        assert!(cap_str.contains("read-file"));
    }

    #[test]
    fn unknown_capability_mapped_to_empty() {
        let cap =
            Capability::new("custom.cap", vec![Platform::Desktop], Scope::Unrestricted).unwrap();
        let perms = capability_permissions(&cap);
        assert!(perms.is_empty());
    }
}
