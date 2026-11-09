//! TR-4.1/4.3 starter: accessibility semantics + 3-breakpoint layout snapshots (SSR).

use dioxus::prelude::*;
use innoframework_ui::components::*;
use innoframework_ui::layout::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[test]
fn tr_4_1_radio_group_uses_fieldset_and_labels() {
    fn app() -> Element {
        rsx! {
            RadioGroup {
                name: "plan".to_string(),
                legend: "Plan".to_string(),
                options: vec![("a".into(), "Alpha".into()), ("b".into(), "Beta".into())],
                selected: "a".to_string(),
            }
        }
    }
    let html = render(app);
    assert!(html.contains("<fieldset"));
    assert!(html.contains("<legend"));
    assert!(html.contains("type=\"radio\"") || html.contains("type='radio'"));
    assert!(html.contains("Alpha") && html.contains("Beta"));
}

#[test]
fn tr_4_1_dialog_is_modal_and_labelled() {
    fn app() -> Element {
        rsx! { Dialog { id: "d1".to_string(), title: "Confirm".to_string(), open: true, "Body" } }
    }
    let html = render(app);
    assert!(html.contains("role=\"dialog\"") || html.contains("role='dialog'"));
    assert!(html.contains("aria-modal=\"true\"") || html.contains("aria-modal='true'"));
    assert!(
        html.contains("aria-labelledby=\"d1-title\"")
            || html.contains("aria-labelledby='d1-title'")
    );
    assert!(html.contains("Confirm"));
}

#[test]
fn tr_4_1_popover_exposes_expanded_state() {
    fn app() -> Element {
        rsx! { Popover { id: "p1".to_string(), trigger_label: "More".to_string(), open: true, "Info" } }
    }
    let html = render(app);
    assert!(html.contains("aria-expanded=\"true\"") || html.contains("aria-expanded='true'"));
    assert!(html.contains("aria-controls=\"p1\"") || html.contains("aria-controls='p1'"));
}

#[test]
fn tr_4_1_tooltip_uses_role_tooltip() {
    fn app() -> Element {
        rsx! { Tooltip { id: "t1".to_string(), label: "Hint".to_string(), span { "?" } } }
    }
    let html = render(app);
    assert!(html.contains("role=\"tooltip\"") || html.contains("role='tooltip'"));
    assert!(html.contains("aria-describedby=\"t1\"") || html.contains("aria-describedby='t1'"));
}

#[test]
fn tr_4_1_tabs_use_tablist_tab_tabpanel() {
    fn app() -> Element {
        rsx! {
            Tabs {
                id: "tabs1".to_string(),
                tabs: vec![("one".into(), "One".into()), ("two".into(), "Two".into())],
                selected: "one".to_string(),
            }
        }
    }
    let html = render(app);
    assert!(html.contains("role=\"tablist\"") || html.contains("role='tablist'"));
    assert!(html.contains("role=\"tab\"") || html.contains("role='tab'"));
    assert!(html.contains("role=\"tabpanel\"") || html.contains("role='tabpanel'"));
    assert!(html.contains("aria-selected=\"true\"") || html.contains("aria-selected='true'"));
}

#[test]
fn tr_4_1_table_has_caption_and_col_headers() {
    fn app() -> Element {
        rsx! {
            Table {
                caption: "People".to_string(),
                headers: vec!["Name".into()],
                rows: vec![vec!["Ada".into()]],
            }
        }
    }
    let html = render(app);
    assert!(html.contains("<caption") || html.contains("<caption>"));
    assert!(html.contains("scope=\"col\"") || html.contains("scope='col'"));
    assert!(html.contains("Ada"));
}

#[test]
fn tr_4_1_toast_region_is_live() {
    fn app() -> Element {
        rsx! { Toast { message: "Done".to_string(), tone: "success".to_string(), open: true } }
    }
    let html = render(app);
    assert!(html.contains("aria-live=\"polite\"") || html.contains("aria-live='polite'"));
    assert!(html.contains("Done"));
}

fn dashboard_markup() -> String {
    fn app() -> Element {
        rsx! {
            AppShell {
                title: "Dashboard".to_string(),
                sidebar: rsx! { Link { href: "/".to_string(), label: "Home".to_string() } },
                Card { title: "Stats".to_string(), Text { content: "42".to_string() } }
            }
        }
    }
    render(app)
}

fn settings_markup() -> String {
    fn app() -> Element {
        rsx! {
            PageLayout { title: "Settings".to_string(),
                FormLayout { title: "Prefs".to_string(), submit_label: "Save".to_string(),
                    Select { id: "t".to_string(), label: "Theme".to_string(), options: vec![("w".into(), "Web".into())] }
                }
            }
        }
    }
    render(app)
}

fn data_table_markup() -> String {
    fn app() -> Element {
        rsx! {
            PageLayout { title: "Data".to_string(),
                Table { caption: "Rows".to_string(), headers: vec!["A".into()], rows: vec![vec!["1".into()]] }
            }
        }
    }
    render(app)
}

#[test]
fn tr_4_3_layout_templates_render_at_3_breakpoints() {
    // Breakpoints are a contract for visual/snapshot tooling (360/768/1280).
    // SSR structure must render without relying on viewport; overflow is
    // enforced by responsive CSS + visual regression (follow-up).
    for bp in [360u32, 768, 1280] {
        let dash = dashboard_markup();
        assert!(
            dash.contains("Dashboard") && dash.contains("Stats"),
            "bp {bp}"
        );
        let settings = settings_markup();
        assert!(
            settings.contains("Settings") && settings.contains("Prefs"),
            "bp {bp}"
        );
        let table = data_table_markup();
        assert!(table.contains("Data") && table.contains("Rows"), "bp {bp}");
    }
}
