use dioxus::prelude::*;

static UI_CSS: Asset = asset!("/assets/inno-ui.css");

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: UI_CSS }
        document::Title { "Showcase 3 — Innoframework Mobile" }
        main { class: "showcase-page", role: "main",
            header { class: "showcase-header",
                h1 { "Showcase #3" }
                p { class: "showcase-subtitle",
                    "Mobile app built with Innoframework + Tauri 2. Demonstrates mobile capabilities and signing identity."
                }
            }
            section { class: "showcase-grid",
                article { class: "showcase-card",
                    h3 { "Keychain" }
                    p { "Declared keychain.get/keychain.set capabilities for secure credential storage." }
                    button { class: "btn btn-primary", onclick: move |_| check_caps(), "Check Capabilities" }
                    div { class: "caps-result", "{caps_status()}" }
                }
                article { class: "showcase-card",
                    h3 { "Dialogs" }
                    p { "Declared dialog.open/dialog.save capabilities for file pickers." }
                    button { class: "btn btn-outline", onclick: move |_| toggle_dialog(), "Toggle Dialog" }
                    div { class: "dialog-status", "{dialog_status()}" }
                }
                article { class: "showcase-card",
                    h3 { "Signing Identity" }
                    p { "Mobile builds require a signing identity (ADR-09 deferred — unsigned bundle)." }
                    button { class: "btn btn-outline", "View Identity" }
                    div { class: "sign-status", "{sign_status()}" }
                }
            }
            footer { class: "showcase-footer",
                p { "Built with Innoframework • Rust + Dioxus 0.7 + Tauri 2 (mobile)" }
                p { class: "note", "Target: android (aab). Signing: deferred (ADR-09)." }
            }
        }
    }
}

#[allow(unused)]
fn check_caps() {}

#[allow(unused)]
fn caps_status() -> String {
    "keychain.get, keychain.set, dialog.open, dialog.save".to_string()
}

#[allow(unused)]
fn toggle_dialog() {}

#[allow(unused)]
fn dialog_status() -> String {
    "Dialog capabilities declared".to_string()
}

#[allow(unused)]
fn sign_status() -> String {
    "showcase3-dev-cert (deferred)".to_string()
}

fn main() {
    LaunchBuilder::mobile().launch(app);
}
