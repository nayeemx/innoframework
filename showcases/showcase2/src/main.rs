use dioxus::prelude::*;

static UI_CSS: Asset = asset!("/assets/inno-ui.css");

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: UI_CSS }
        document::Title { "Showcase 2 — Innoframework Desktop" }
        main { class: "showcase-page", role: "main",
            header { class: "showcase-header",
                h1 { "Showcase #2" }
                p { class: "showcase-subtitle",
                    "Desktop app built with Innoframework + Tauri 2. Demonstrates desktop capabilities and auth guards."
                }
            }
            section { class: "showcase-grid",
                article { class: "showcase-card",
                    h3 { "File System Access" }
                    p { "Declared fs.read/fs.write capabilities for data/ and logs/ directories." }
                    button { class: "btn btn-primary", onclick: move |_| check_caps(), "Check Capabilities" }
                    div { class: "caps-result", "{caps_status()}" }
                }
                article { class: "showcase-card",
                    h3 { "Auth Guard" }
                    p { "Route protected by fail-closed auth guard (role: user)." }
                    button { class: "btn btn-outline", onclick: move |_| toggle_auth(), "Toggle Auth" }
                    div { class: "auth-status", "{auth_status()}" }
                }
                article { class: "showcase-card",
                    h3 { "Notifications" }
                    p { "notify.send capability enables desktop notifications." }
                    button { class: "btn btn-outline", onclick: move |_| send_notification(), "Send Notification" }
                    div { class: "notify-status", "{notify_status()}" }
                }
            }
            footer { class: "showcase-footer",
                p { "Built with Innoframework • Rust + Dioxus 0.7 + Tauri 2" }
                p { class: "note", "Signing: deferred (ADR-09) — unsigned desktop bundle." }
            }
        }
    }
}

#[allow(unused)]
fn check_caps() {}

#[allow(unused)]
fn caps_status() -> String {
    "fs.read (data/), fs.write (data/, logs/), notify.send".to_string()
}

#[allow(unused)]
fn toggle_auth() {}

#[allow(unused)]
fn auth_status() -> String {
    "Authenticated: demo-user (role: user)".to_string()
}

#[allow(unused)]
fn send_notification() {}

#[allow(unused)]
fn notify_status() -> String {
    "Notification capability declared".to_string()
}

fn main() {
    LaunchBuilder::desktop().launch(app);
}
