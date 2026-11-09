//! Base CSS for P4 UI kit (CSP-safe external stylesheet, TR-4.1-4.5 baseline).
//!
//! Paired with `tokens::Tokens::to_css_vars()` (theme variables). Served/wired by
//! `inno init` templates via `asset!("/assets/inno-ui.css")`.

pub const BASE_CSS: &str = r#"/* Innoframework UI kit (P4) - CSP-safe external stylesheet */
*,*::before,*::after{box-sizing:border-box}
html,body{margin:0;padding:0}
body{font-family:var(--inno-font-sans);font-size:var(--inno-size-md);line-height:var(--inno-line-height);color:var(--inno-fg);background:var(--inno-bg)}
:focus-visible{outline:var(--inno-focus-width) solid var(--inno-focus-ring);outline-offset:var(--inno-focus-offset)}
.inno-skip-link{position:absolute;left:-999px;top:auto;width:1px;height:1px;overflow:hidden}
.inno-skip-link:focus{left:var(--inno-space-sm);top:var(--inno-space-sm);width:auto;height:auto;padding:var(--inno-space-sm);background:var(--inno-bg-elevated);z-index:1000}
.inno-page,.inno-container{max-width:960px;margin:0 auto;padding:var(--inno-space-lg)}
.inno-page__header,.inno-app-shell__header{margin-bottom:var(--inno-space-lg)}
.inno-page__content,.inno-card__body,.inno-form,.inno-field{display:grid;gap:var(--inno-space-md)}
.inno-card{background:var(--inno-bg-elevated);border:1px solid var(--inno-border);border-radius:var(--inno-radius-md);padding:var(--inno-space-lg)}
.inno-input,.inno-select,.inno-textarea{font:inherit;color:var(--inno-fg);background:var(--inno-bg);border:1px solid var(--inno-border);border-radius:var(--inno-radius-sm);padding:10px 12px;min-height:44px;width:100%}
.inno-textarea{min-height:96px}
.inno-button{font:inherit;font-weight:600;border:1px solid var(--inno-border);border-radius:var(--inno-radius-sm);padding:10px 14px;min-height:44px;cursor:pointer;background:var(--inno-bg-elevated);color:var(--inno-fg)}
.inno-button--primary{background:var(--inno-primary);color:var(--inno-primary-fg);border-color:transparent}
.inno-button[disabled],.inno-button[aria-disabled="true"]{opacity:.55;cursor:not-allowed}
.inno-form__actions{display:flex;gap:var(--inno-space-sm);flex-wrap:wrap}
.inno-text--muted{color:var(--inno-fg-muted)}
.inno-alert{padding:var(--inno-space-md);border-radius:var(--inno-radius-sm);border:1px solid var(--inno-border)}
.inno-alert--danger{border-color:var(--inno-danger);color:var(--inno-danger)}
.inno-alert--success{border-color:var(--inno-success);color:var(--inno-success)}
.inno-table{width:100%;border-collapse:collapse}
.inno-table th,.inno-table td{border:1px solid var(--inno-border);padding:8px;text-align:left}
.inno-tabs [role="tablist"]{display:flex;gap:var(--inno-space-xs);flex-wrap:wrap}
.inno-tab{min-height:44px}
.inno-app-shell__body{display:grid;gap:var(--inno-space-lg)}
@media (max-width: 360px){.inno-page,.inno-container{padding:var(--inno-space-sm)}.inno-card{padding:var(--inno-space-md)}}
@media (min-width: 768px){.inno-page,.inno-container{padding:var(--inno-space-xl)}.inno-page__content{grid-template-columns:repeat(2,minmax(0,1fr))}.inno-app-shell__body{grid-template-columns:220px 1fr}}
@media (min-width: 1280px){.inno-page,.inno-container{max-width:1200px}.inno-page__content{grid-template-columns:repeat(3,minmax(0,1fr))}}
@media (prefers-reduced-motion: reduce){*{scroll-behavior:auto}}
"#;

pub fn css_with_theme_vars(vars: &str) -> String {
    format!("{vars}\n{BASE_CSS}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_css_includes_breakpoints_and_focus_rules() {
        assert!(BASE_CSS.contains("@media (max-width: 360px)"));
        assert!(BASE_CSS.contains("@media (min-width: 768px)"));
        assert!(BASE_CSS.contains("@media (min-width: 1280px)"));
        assert!(BASE_CSS.contains(":focus-visible"));
        assert!(BASE_CSS.contains("prefers-reduced-motion"));
    }
}
