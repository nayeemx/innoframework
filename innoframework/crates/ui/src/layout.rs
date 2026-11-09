//! Layout templates (P4 starter set).

use dioxus::prelude::*;

#[component]
pub fn AppShell(title: String, sidebar: Element, children: Element) -> Element {
    rsx! {
        div { class: "inno-app-shell",
            a { class: "inno-skip-link", href: "#main-content", "Skip to main content" }
            header { class: "inno-app-shell__header", role: "banner", h1 { "{title}" } }
            div { class: "inno-app-shell__body",
                nav { class: "inno-app-shell__sidebar", "aria-label": "Primary", {sidebar} }
                main { id: "main-content", class: "inno-app-shell__main", role: "main", {children} }
            }
        }
    }
}

#[component]
pub fn PageLayout(title: String, children: Element) -> Element {
    rsx! {
        main { class: "inno-page", role: "main",
            header { class: "inno-page__header", h1 { "{title}" } }
            section { class: "inno-page__content", {children} }
        }
    }
}

#[component]
pub fn FormLayout(
    title: String,
    children: Element,
    #[props(default)] submit_label: String,
) -> Element {
    let submit = if submit_label.is_empty() {
        "Submit".into()
    } else {
        submit_label
    };
    rsx! {
        section { class: "inno-form-layout", "aria-labelledby": "form-title",
            h1 { id: "form-title", "{title}" }
            form { class: "inno-form", novalidate: false,
                {children}
                div { class: "inno-form__actions",
                    button { r#type: "submit", class: "inno-button inno-button--primary", "{submit}" }
                }
            }
        }
    }
}
