//! Innoframework UI kit (P4 scaffold).
//!
//! Provides a token system, theme presets (Web/Windows), layout primitives,
//! and an initial accessible component set. Components render via Dioxus 0.7
//! `rsx!` and are snapshot-tested with `dioxus-ssr` (TR-4.x/visual baselines).

pub mod a11y;
pub mod components;
pub mod css;
pub mod layout;
pub mod theme;
pub mod tokens;

pub use components::{
    Alert, Badge, Button, Card, Checkbox, Container, Dialog, Divider, Form, Heading, Input, Link,
    Popover, RadioGroup, Select, Stack, Switch, Table, Tabs, Text, TextArea, Toast, Tooltip,
};
pub use css::BASE_CSS;
pub use layout::{AppShell, FormLayout, PageLayout};
pub use theme::{Theme, ThemeName};
pub use tokens::Tokens;
