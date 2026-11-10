//! SSR render + accessibility/structure snapshots (P4 scaffold, TR-4.x starter).

use dioxus::prelude::*;
use innoframework_ui::components::*;
use innoframework_ui::layout::*;
use innoframework_ui::theme::Theme;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn theme_css_vars_present_for_web_and_windows() {
    let web = Theme::web().css();
    assert!(web.contains("--inno-primary:#0969da"));
    let win = Theme::windows().css();
    assert!(win.contains("--inno-primary:#0067c0"));
    assert!(win.contains("Segoe UI"));
}

#[test]
fn button_renders_accessible_attributes() {
    fn app() -> Element {
        rsx! { Button { label: "Save".to_string(), variant: "primary".to_string(), disabled: false, button_type: "submit".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("inno-button--primary"));
    assert!(html.contains("type=\"submit\"") || html.contains("type='submit'"));
    assert!(html.contains("Save"));
    assert!(html.contains("aria-disabled"));
}

#[test]
fn input_is_labelled_and_reports_invalid_state() {
    fn app() -> Element {
        rsx! { Input { id: "email".to_string(), label: "Email".to_string(), input_type: "email".to_string(), required: true, invalid: true, described_by: "email-err".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("for=\"email\"") || html.contains("for='email'"));
    assert!(html.contains("aria-invalid=\"true\"") || html.contains("aria-invalid='true'"));
    assert!(html.contains("aria-required=\"true\"") || html.contains("aria-required='true'"));
    assert!(
        html.contains("aria-describedby=\"email-err\"")
            || html.contains("aria-describedby='email-err'")
    );
}

#[test]
fn switch_exposes_switch_role_and_state() {
    fn app() -> Element {
        rsx! { Switch { id: "notify".to_string(), label: "Notifications".to_string(), on: true } }
    }
    let html = render(app);
    assert!(html.contains("role=\"switch\"") || html.contains("role='switch'"));
    assert!(html.contains("aria-checked=\"true\"") || html.contains("aria-checked='true'"));
}

#[test]
fn alert_uses_live_region_role() {
    fn app() -> Element {
        rsx! { Alert { message: "Saved".to_string(), tone: "success".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("role=\"status\"") || html.contains("role='status'"));
    assert!(html.contains("aria-live=\"polite\"") || html.contains("aria-live='polite'"));
}

#[test]
fn dialog_reflects_open_state() {
    fn open_app() -> Element {
        rsx! { Dialog { id: "d1".to_string(), title: "Confirm".to_string(), open: true, "Body" } }
    }
    let html = render(open_app);
    assert!(html.contains("aria-hidden=\"false\"") || html.contains("aria-hidden='false'"));
    assert!(!html.contains(" hidden"));
}

#[test]
fn heading_renders_requested_level() {
    fn app() -> Element {
        rsx! { Heading { text: "Sub".to_string(), level: 2 } }
    }
    let html = render(app);
    assert!(html.contains("<h2"));
    assert!(html.contains("inno-heading--2"));
}

#[test]
fn form_layout_has_labelled_region_and_submit() {
    fn app() -> Element {
        rsx! {
            FormLayout { title: "Profile".to_string(), submit_label: "Create".to_string(),
                Input { id: "name".to_string(), label: "Name".to_string() }
            }
        }
    }
    let html = render(app);
    assert!(
        html.contains("aria-labelledby=\"form-title\"")
            || html.contains("aria-labelledby='form-title'")
    );
    assert!(html.contains("Create"));
    assert!(html.contains("Name"));
}

#[test]
fn app_shell_has_landmarks_and_skip_link() {
    fn app() -> Element {
        rsx! {
            AppShell {
                title: "Demo".to_string(),
                sidebar: rsx! { Link { href: "/".to_string(), label: "Home".to_string() } },
                Container { Text { content: "Body".to_string() } }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("Skip to main content"));
    assert!(html.contains("role=\"main\"") || html.contains("id=\"main-content\""));
    assert!(html.contains("aria-label=\"Primary\"") || html.contains("aria-label='Primary'"));
}

#[test]
fn visual_snapshot_smoke_form_page() {
    fn app() -> Element {
        rsx! {
            PageLayout { title: "Settings".to_string(),
                Stack { gap: "md".to_string(),
                    Card { title: "Account".to_string(),
                        Input { id: "u".to_string(), label: "Username".to_string(), value: "alice".to_string() }
                        Checkbox { id: "t".to_string(), label: "Accept terms".to_string(), checked: true }
                    }
                    Alert { message: "Check fields".to_string(), tone: "warning".to_string() }
                }
            }
        }
    }
    let html = render(app);
    let expected = concat!(
        "Settings",
        "|",
        "Account",
        "|",
        "Username",
        "|",
        "alice",
        "|",
        "Accept terms",
        "|",
        "Check fields"
    );
    for part in expected.split('|') {
        assert!(html.contains(part), "snapshot missing: {part}\n{html}");
    }
}

#[test]
fn textarea_is_labelled_and_required() {
    fn app() -> Element {
        rsx! { TextArea { id: "bio".to_string(), label: "Bio".to_string(), required: true } }
    }
    let html = render(app);
    assert!(html.contains("for=\"bio\"") || html.contains("for='bio'"));
    assert!(html.contains("aria-required=\"true\"") || html.contains("aria-required='true'"));
    assert!(html.contains("Bio"));
}

#[test]
fn checkbox_has_label_and_checked_state() {
    fn app() -> Element {
        rsx! { Checkbox { id: "agree".to_string(), label: "Agree".to_string(), checked: true } }
    }
    let html = render(app);
    assert!(html.contains("type=\"checkbox\"") || html.contains("type='checkbox'"));
    assert!(html.contains("for=\"agree\"") || html.contains("for='agree'"));
    assert!(html.contains("checked"));
    assert!(html.contains("Agree"));
}

#[test]
fn select_renders_options_and_selected() {
    fn app() -> Element {
        rsx! { Select { id: "role".to_string(), label: "Role".to_string(), options: vec![("admin".into(), "Admin".into()), ("viewer".into(), "Viewer".into())], selected: "admin".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("Admin") && html.contains("Viewer"));
    assert!(html.contains("value=\"admin\"") || html.contains("value='admin'"));
}

#[test]
fn disabled_button_reports_aria_disabled() {
    fn app() -> Element {
        rsx! { Button { label: "Submit".to_string(), disabled: true, button_type: "submit".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("aria-disabled=\"true\"") || html.contains("aria-disabled='true'"));
    assert!(html.contains("Submit"));
}

#[test]
fn invalid_input_reports_aria_invalid() {
    fn app() -> Element {
        rsx! { Input { id: "pin".to_string(), label: "PIN".to_string(), invalid: true } }
    }
    let html = render(app);
    assert!(html.contains("aria-invalid=\"true\"") || html.contains("aria-invalid='true'"));
    assert!(html.contains("PIN"));
}

#[test]
fn input_passes_autocomplete_through() {
    fn app() -> Element {
        rsx! { Input { id: "email".to_string(), label: "Email".to_string(), autocomplete: "email".to_string() } }
    }
    let html = render(app);
    assert!(html.contains("autocomplete=\"email\"") || html.contains("autocomplete='email'"));
}

#[test]
fn popover_exposes_expanded_and_controls() {
    fn app() -> Element {
        rsx! { Popover { id: "more".to_string(), trigger_label: "Options".to_string(), open: true, "Settings" } }
    }
    let html = render(app);
    assert!(html.contains("aria-expanded=\"true\"") || html.contains("aria-expanded='true'"));
    assert!(html.contains("aria-controls=\"more\"") || html.contains("aria-controls='more'"));
    assert!(html.contains("Options"));
}
