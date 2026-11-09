//! Theme selection and CSS emission.

use crate::tokens::Tokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeName {
    Web,
    Windows,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: ThemeName,
    pub tokens: Tokens,
}

impl Theme {
    pub fn web() -> Self {
        Self {
            name: ThemeName::Web,
            tokens: Tokens::web(),
        }
    }

    pub fn windows() -> Self {
        Self {
            name: ThemeName::Windows,
            tokens: Tokens::windows(),
        }
    }

    pub fn css(&self) -> String {
        self.tokens.to_css_vars()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themes_emit_css_vars() {
        assert!(Theme::web().css().starts_with(":root{--inno-bg"));
        assert!(Theme::windows().css().contains("--inno-font-sans"));
    }
}
