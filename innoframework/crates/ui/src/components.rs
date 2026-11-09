//! Accessible component primitives (P4 initial set).
//!
//! These are render-only building blocks with explicit accessible attributes
//! (labels, roles, aria-* where appropriate). Behavior/state wiring is owned by
//! the app; components focus on semantics and token-driven styling.

use dioxus::prelude::*;

#[component]
pub fn Button(
    label: String,
    #[props(default)] variant: String,
    #[props(default)] disabled: bool,
    #[props(default)] button_type: String,
) -> Element {
    let kind = if variant.is_empty() {
        "primary".to_string()
    } else {
        variant
    };
    let ty = if button_type.is_empty() {
        "button".to_string()
    } else {
        button_type
    };
    rsx! {
        button {
            r#type: "{ty}",
            class: "inno-button inno-button--{kind}",
            disabled: disabled,
            "aria-disabled": if disabled { "true" } else { "false" },
            "{label}"
        }
    }
}

#[component]
pub fn Input(
    id: String,
    label: String,
    #[props(default)] value: String,
    #[props(default)] input_type: String,
    #[props(default)] required: bool,
    #[props(default)] invalid: bool,
    #[props(default)] described_by: String,
) -> Element {
    let ty = if input_type.is_empty() {
        "text".to_string()
    } else {
        input_type
    };
    rsx! {
        div { class: "inno-field",
            label { r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                name: "{id}",
                class: "inno-input",
                r#type: "{ty}",
                value: "{value}",
                required: required,
                "aria-required": if required { "true" } else { "false" },
                "aria-invalid": if invalid { "true" } else { "false" },
                "aria-describedby": if described_by.is_empty() { None } else { Some(described_by) },
            }
        }
    }
}

#[component]
pub fn TextArea(
    id: String,
    label: String,
    #[props(default)] value: String,
    #[props(default)] required: bool,
) -> Element {
    rsx! {
        div { class: "inno-field",
            label { r#for: "{id}", "{label}" }
            textarea {
                id: "{id}",
                name: "{id}",
                class: "inno-textarea",
                required: required,
                "aria-required": if required { "true" } else { "false" },
                "{value}"
            }
        }
    }
}

#[component]
pub fn Select(
    id: String,
    label: String,
    options: Vec<(String, String)>,
    #[props(default)] selected: String,
) -> Element {
    rsx! {
        div { class: "inno-field",
            label { r#for: "{id}", "{label}" }
            select { id: "{id}", name: "{id}", class: "inno-select",
                for (value , text) in options {
                    option { value: "{value}", selected: value == selected, "{text}" }
                }
            }
        }
    }
}

#[component]
pub fn Checkbox(id: String, label: String, #[props(default)] checked: bool) -> Element {
    rsx! {
        div { class: "inno-check",
            input { id: "{id}", name: "{id}", r#type: "checkbox", checked: checked }
            label { r#for: "{id}", "{label}" }
        }
    }
}

#[component]
pub fn Switch(id: String, label: String, #[props(default)] on: bool) -> Element {
    rsx! {
        div { class: "inno-switch",
            button {
                id: "{id}",
                r#type: "button",
                role: "switch",
                "aria-checked": if on { "true" } else { "false" },
                "aria-label": "{label}",
                class: if on { "inno-switch__control is-on" } else { "inno-switch__control" },
            }
            label { r#for: "{id}", "{label}" }
        }
    }
}

#[component]
pub fn Alert(message: String, #[props(default)] tone: String) -> Element {
    let kind = if tone.is_empty() { "info".into() } else { tone };
    let role = if kind == "danger" { "alert" } else { "status" };
    rsx! {
        div { class: "inno-alert inno-alert--{kind}", role: "{role}", "aria-live": "polite", "{message}" }
    }
}

#[component]
pub fn Badge(text: String, #[props(default)] tone: String) -> Element {
    let kind = if tone.is_empty() {
        "neutral".into()
    } else {
        tone
    };
    rsx! { span { class: "inno-badge inno-badge--{kind}", "{text}" } }
}

#[component]
pub fn Card(title: String, children: Element) -> Element {
    rsx! {
        section { class: "inno-card", "aria-label": "{title}",
            header { class: "inno-card__header", h2 { class: "inno-card__title", "{title}" } }
            div { class: "inno-card__body", {children} }
        }
    }
}

#[component]
pub fn Heading(text: String, #[props(default = 1)] level: u8) -> Element {
    let tag = match level {
        2 => "h2",
        3 => "h3",
        4 => "h4",
        _ => "h1",
    };
    rsx! { h1 { class: "inno-heading inno-heading--{level}", "data-tag": "{tag}", "{text}" } }
}

#[component]
pub fn Text(content: String, #[props(default)] muted: bool) -> Element {
    let cls = if muted {
        "inno-text inno-text--muted"
    } else {
        "inno-text"
    };
    rsx! { p { class: "{cls}", "{content}" } }
}

#[component]
pub fn Link(href: String, label: String) -> Element {
    rsx! { a { class: "inno-link", href: "{href}", "{label}" } }
}

#[component]
pub fn Divider() -> Element {
    rsx! { hr { class: "inno-divider", role: "separator" } }
}

#[component]
pub fn Stack(children: Element, #[props(default)] gap: String) -> Element {
    let g = if gap.is_empty() { "md".into() } else { gap };
    rsx! { div { class: "inno-stack inno-stack--gap-{g}", {children} } }
}

#[component]
pub fn Container(children: Element) -> Element {
    rsx! { div { class: "inno-container", {children} } }
}
