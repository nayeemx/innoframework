//! TR-4.2 token diff / theme rules (P4).
//!
//! Contract: Web and Windows themes share the same token keys; values may
//! differ, but structure must not (components stay theme-agnostic).

use innoframework_ui::theme::{Theme, ThemeName};
use innoframework_ui::tokens::Tokens;

#[test]
fn tr_4_2_theme_names_are_distinct() {
    assert_ne!(Theme::web().name, Theme::windows().name);
    assert_eq!(Theme::web().name, ThemeName::Web);
    assert_eq!(Theme::windows().name, ThemeName::Windows);
}

#[test]
fn tr_4_2_token_structure_present_in_both_themes() {
    for t in [Tokens::web(), Tokens::windows()] {
        let css = t.to_css_vars();
        for key in [
            "--inno-bg",
            "--inno-fg",
            "--inno-primary",
            "--inno-danger",
            "--inno-focus-ring",
            "--inno-space-md",
            "--inno-radius-md",
            "--inno-font-sans",
            "--inno-focus-width",
        ] {
            assert!(css.contains(key), "missing token {key}");
        }
    }
}

#[test]
fn tr_4_2_platform_theme_changes_values_not_contract() {
    let web = Tokens::web();
    let win = Tokens::windows();
    // Values may differ per platform pack.
    assert_ne!(web.color.primary, win.color.primary);
    assert_ne!(web.type_scale.font_sans, win.type_scale.font_sans);
    // Shared structural invariants remain meaningful.
    assert!(!web.color.bg.is_empty());
    assert!(!win.color.bg.is_empty());
}
