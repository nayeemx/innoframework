//! Tauri 2 desktop build integration (P5, ADR-02).
//!
//! Generates Tauri project files (`tauri.conf.json`, capability JSONs) from
//! the project manifest, then runs `cargo tauri build` to produce the
//! desktop bundle. Capabilities declared in `inno.toml` are mapped to Tauri
//! permissions and allowlisted commands — fail closed: undeclared native
//! APIs are not exposed (TR-3.2).

use crate::errors::InnoframeworkCliError;
use innoframework_core::capability::{Capability, Platform, Scope};
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;

pub const TAURI_DIR: &str = "src-tauri";
pub const CAPABILITIES_DIR: &str = "capabilities";

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

/// Generate `tauri.conf.json` from manifest capabilities.
pub fn generate_tauri_conf(
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
) -> Value {
    let mut allowlist = Map::new();

    let mut notification = Map::new();
    let mut keychain = Map::new();
    let mut dialog = Map::new();
    let mut shell = Map::new();

    let mut tauri_capabilities = Vec::new();

    for cap in capabilities {
        if !cap.platforms().contains(&Platform::Desktop) {
            continue;
        }
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
        if !cap.platforms().contains(&Platform::Desktop) {
            continue;
        }
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
        "app": {
            "windows": [
                {
                    "title": project_name,
                    "width": 800,
                    "height": 600,
                    "resizable": true,
                }
            ],
            "security": {
                "csp": csp,
            },
        },
        "tauri": {
            "allowlist": allowlist,
            "capabilities": tauri_capabilities,
            "bundle": {
                "active": true,
                "targets": "all",
                "identifier": format!("com.innoframework.{}", project_name),
                "icon": [
                    "icons/32x32.png",
                    "icons/128x128.png",
                    "icons/128x128@2x.png",
                    "icons/icon.icns",
                    "icons/icon.ico",
                ],
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

/// Write Tauri project files to `src-tauri/`.
pub fn write_tauri_project(
    root: &Path,
    project_name: &str,
    version: &str,
    capabilities: &[Capability],
    csp: &str,
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
    let conf = generate_tauri_conf(project_name, version, capabilities, csp);
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

    // Write capability JSON files.
    for cap in capabilities {
        if !cap.platforms().contains(&Platform::Desktop) {
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

    #[test]
    fn generate_tauri_conf_includes_capabilities() {
        let caps = vec![fs_read_desktop(), notify_send_desktop()];
        let conf = generate_tauri_conf("my-app", "0.1.0", &caps, "default-src 'self'");
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
        let conf = generate_tauri_conf("my-app", "0.1.0", &caps, "default-src 'self'");
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
        let conf = generate_tauri_conf("my-app", "0.1.0", &caps, "default-src 'self'");
        let conf_str = serde_json::to_string(&conf).unwrap();
        assert!(conf_str.contains("data/"));
        assert!(conf_str.contains("logs/"));
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
