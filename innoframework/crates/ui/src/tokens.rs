//! Design tokens (spacing, type scale, radii, semantic roles).
//!
//! Tokens are the single source of truth; themes map semantic roles to values.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Tokens {
    pub color: ColorTokens,
    pub space: SpaceTokens,
    pub radius: RadiusTokens,
    pub type_scale: TypeTokens,
    pub focus: FocusTokens,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ColorTokens {
    pub bg: &'static str,
    pub bg_elevated: &'static str,
    pub fg: &'static str,
    pub fg_muted: &'static str,
    pub border: &'static str,
    pub primary: &'static str,
    pub primary_fg: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub focus_ring: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SpaceTokens {
    pub xxs: &'static str,
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RadiusTokens {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub pill: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TypeTokens {
    pub font_sans: &'static str,
    pub size_sm: &'static str,
    pub size_md: &'static str,
    pub size_lg: &'static str,
    pub size_xl: &'static str,
    pub weight_normal: &'static str,
    pub weight_semibold: &'static str,
    pub line_height: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FocusTokens {
    pub ring_width: &'static str,
    pub ring_offset: &'static str,
}

impl Tokens {
    pub const fn web() -> Self {
        Self {
            color: ColorTokens {
                bg: "#ffffff",
                bg_elevated: "#f6f8fa",
                fg: "#1f2328",
                fg_muted: "#59636e",
                border: "#d1d9e0",
                primary: "#0969da",
                primary_fg: "#ffffff",
                danger: "#d1242f",
                danger_fg: "#ffffff",
                success: "#1a7f37",
                warning: "#9a6700",
                focus_ring: "#0969da",
            },
            space: SpaceTokens {
                xxs: "2px",
                xs: "4px",
                sm: "8px",
                md: "12px",
                lg: "16px",
                xl: "24px",
            },
            radius: RadiusTokens {
                sm: "4px",
                md: "6px",
                lg: "10px",
                pill: "999px",
            },
            type_scale: TypeTokens {
                font_sans: "system-ui, -apple-system, Segoe UI, sans-serif",
                size_sm: "12px",
                size_md: "14px",
                size_lg: "16px",
                size_xl: "20px",
                weight_normal: "400",
                weight_semibold: "600",
                line_height: "1.5",
            },
            focus: FocusTokens {
                ring_width: "2px",
                ring_offset: "2px",
            },
        }
    }

    pub const fn windows() -> Self {
        Self {
            color: ColorTokens {
                bg: "#f3f3f3",
                bg_elevated: "#ffffff",
                fg: "#1a1a1a",
                fg_muted: "#5d5d5d",
                border: "#e5e5e5",
                primary: "#0067c0",
                primary_fg: "#ffffff",
                danger: "#c42b1c",
                danger_fg: "#ffffff",
                success: "#0f7b0f",
                warning: "#9d5d00",
                focus_ring: "#000000",
            },
            space: SpaceTokens {
                xxs: "2px",
                xs: "4px",
                sm: "8px",
                md: "12px",
                lg: "16px",
                xl: "24px",
            },
            radius: RadiusTokens {
                sm: "4px",
                md: "4px",
                lg: "8px",
                pill: "999px",
            },
            type_scale: TypeTokens {
                font_sans: "'Segoe UI Variable', 'Segoe UI', system-ui, sans-serif",
                size_sm: "12px",
                size_md: "14px",
                size_lg: "16px",
                size_xl: "20px",
                weight_normal: "400",
                weight_semibold: "600",
                line_height: "1.4",
            },
            focus: FocusTokens {
                ring_width: "2px",
                ring_offset: "1px",
            },
        }
    }

    /// Serialize tokens to CSS custom properties for `:root` injection.
    pub fn to_css_vars(&self) -> String {
        format!(
            ":root{{--inno-bg:{};--inno-bg-elevated:{};--inno-fg:{};--inno-fg-muted:{};--inno-border:{};--inno-primary:{};--inno-primary-fg:{};--inno-danger:{};--inno-danger-fg:{};--inno-success:{};--inno-warning:{};--inno-focus-ring:{};--inno-space-xs:{};--inno-space-sm:{};--inno-space-md:{};--inno-space-lg:{};--inno-space-xl:{};--inno-radius-sm:{};--inno-radius-md:{};--inno-radius-lg:{};--inno-font-sans:{};--inno-size-sm:{};--inno-size-md:{};--inno-size-lg:{};--inno-line-height:{};--inno-focus-width:{};--inno-focus-offset:{}}}",
            self.color.bg,
            self.color.bg_elevated,
            self.color.fg,
            self.color.fg_muted,
            self.color.border,
            self.color.primary,
            self.color.primary_fg,
            self.color.danger,
            self.color.danger_fg,
            self.color.success,
            self.color.warning,
            self.color.focus_ring,
            self.space.xs,
            self.space.sm,
            self.space.md,
            self.space.lg,
            self.space.xl,
            self.radius.sm,
            self.radius.md,
            self.radius.lg,
            self.type_scale.font_sans,
            self.type_scale.size_sm,
            self.type_scale.size_md,
            self.type_scale.size_lg,
            self.type_scale.line_height,
            self.focus.ring_width,
            self.focus.ring_offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn web_tokens_define_required_semantic_colors() {
        let t = Tokens::web();
        assert_eq!(t.color.primary, "#0969da");
        assert!(t.to_css_vars().contains("--inno-focus-ring"));
    }

    #[test]
    fn windows_theme_differs_from_web_primary() {
        assert_ne!(Tokens::web().color.primary, Tokens::windows().color.primary);
    }
}
