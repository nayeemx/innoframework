//! Innoframework UI kit (P4 scaffold).
//!
//! Provides a token system, theme presets (Web/Windows), layout primitives,
//! and an initial accessible component set. Components render via Dioxus 0.7
//! `rsx!` and are snapshot-tested with `dioxus-ssr` (TR-4.x/visual baselines).

pub mod components;
pub mod layout;
pub mod theme;
pub mod tokens;

pub use components::{
    Alert, Badge, Button, Card, Checkbox, Container, Divider, Heading, Input, Link, Select, Stack,
    Switch, Text, TextArea,
};
pub use layout::{AppShell, FormLayout, PageLayout};
pub use theme::{Theme, ThemeName};
pub use tokens::Tokens;
