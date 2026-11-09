//! TR-4.5 design principles (checkable rules) + small lint helpers.

use crate::theme::Theme;

/// TR-4.5 (documented/checkable): contrast >= 4.5:1 for normal text, touch
/// target >= 44px on mobile, focus indicator visible, reduced motion respected.
/// These rules are enforced progressively (token/lint/CI); this module records
/// the contract and provides basic checks where feasible without a browser.
pub const MIN_CONTRAST_NORMAL: f32 = 4.5;
pub const MIN_TOUCH_TARGET_PX: u32 = 44;

pub fn relative_luminance(hex: &str) -> Option<f32> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&h[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&h[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&h[4..6], 16).ok()? as f32 / 255.0;
    let f = |c: f32| {
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    Some(0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b))
}

pub fn contrast_ratio(a: &str, b: &str) -> Option<f32> {
    let la = relative_luminance(a)?;
    let lb = relative_luminance(b)?;
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    Some((hi + 0.05) / (lo + 0.05))
}

pub fn web_text_contrast_ok() -> bool {
    let t = Theme::web();
    contrast_ratio(t.tokens.color.fg, t.tokens.color.bg).is_some_and(|r| r >= MIN_CONTRAST_NORMAL)
        && contrast_ratio(t.tokens.color.primary_fg, t.tokens.color.primary)
            .is_some_and(|r| r >= MIN_CONTRAST_NORMAL)
}

pub fn windows_text_contrast_ok() -> bool {
    let t = Theme::windows();
    contrast_ratio(t.tokens.color.fg, t.tokens.color.bg).is_some_and(|r| r >= MIN_CONTRAST_NORMAL)
        && contrast_ratio(t.tokens.color.primary_fg, t.tokens.color.primary)
            .is_some_and(|r| r >= MIN_CONTRAST_NORMAL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tr_4_5_theme_text_contrast_meets_minimum() {
        assert!(web_text_contrast_ok());
        assert!(windows_text_contrast_ok());
    }

    #[test]
    fn tr_4_5_touch_target_rule_recorded() {
        assert_eq!(MIN_TOUCH_TARGET_PX, 44);
        assert!((MIN_CONTRAST_NORMAL - 4.5).abs() < f32::EPSILON);
    }
}
