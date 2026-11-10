use dioxus::prelude::*;

static UI_CSS: Asset = asset!("/assets/inno-ui.css");

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: UI_CSS }
        document::Title { "Showcase 1 — Innoframework Demo" }
        main { class: "showcase-page", role: "main",
            header { class: "showcase-header",
                h1 { "Showcase #1" }
                p { class: "showcase-subtitle",
                    "Demonstrating Innoframework capabilities: typed server functions, auth guards, default-deny security."
                }
            }
            section { class: "showcase-grid",
                article { class: "showcase-card",
                    h3 { "Typed Server Functions" }
                    p { "Call server-side Rust from the client with full type safety." }
                    button { class: "btn btn-primary", onclick: move |_| call_server(), "Invoke Server Function" }
                    div { class: "server-result", "{server_result()}" }
                }
                article { class: "showcase-card",
                    h3 { "Auth Guard" }
                    p { "Routes protected by fail-closed auth guards." }
                    button { class: "btn btn-outline", onclick: move |_| toggle_auth(), "Toggle Auth" }
                    div { class: "auth-status", "{auth_status()}" }
                }
                article { class: "showcase-card",
                    h3 { "CSP Header" }
                    p { "Content-Security-Policy: default-src 'self'..." }
                    button { class: "btn btn-outline", "View CSP" }
                }
            }
            footer { class: "showcase-footer",
                p { "Built with Innoframework • Rust + Dioxus 0.7" }
            }
        }
    }
}

#[server]
async fn call_server() -> Result<String, ServerFnError> {
    Ok("Server response received.".to_string())
}

#[allow(unused)]
fn server_result() -> String {
    String::new()
}

#[allow(unused)]
fn auth_status() -> String {
    "Authenticated: demo-user (role: user)".to_string()
}

#[allow(unused)]
fn toggle_auth() {}

fn main() {
    LaunchBuilder::web().launch(app);
}
