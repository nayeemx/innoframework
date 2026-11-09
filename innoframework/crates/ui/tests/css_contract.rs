//! TR-4.1/4.3/4.4/4.5 CSS contract: focus, motion, reduced-motion, breakpoints,
//! and touch-target/min-height baselines in the shared stylesheet.

use innoframework_ui::css::{css_with_theme_vars, BASE_CSS};
use innoframework_ui::tokens::Tokens;

#[test]
fn tr_4_3_breakpoints_and_tr_4_4_reduced_motion_present() {
    assert!(BASE_CSS.contains("@media (max-width: 360px)"));
    assert!(BASE_CSS.contains("@media (min-width: 768px)"));
    assert!(BASE_CSS.contains("@media (min-width: 1280px)"));
    assert!(BASE_CSS.contains("prefers-reduced-motion: reduce"));
}

#[test]
fn tr_4_5_focus_visible_and_touch_target_baselines_present() {
    assert!(BASE_CSS.contains(":focus-visible"));
    assert!(BASE_CSS.contains("--inno-focus-ring"));
    // Touch-target baseline (44px min-height on primary controls).
    assert!(BASE_CSS.contains("min-height:44px"));
}

#[test]
fn css_with_theme_includes_motion_vars_and_reduced_motion_override() {
    let css = css_with_theme_vars(&Tokens::web().to_css_vars());
    assert!(css.contains("--inno-duration-normal"));
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
    assert!(css.contains("--inno-duration-normal:0ms"));
    assert!(css.contains("outline:var(--inno-focus-width)"));
}
