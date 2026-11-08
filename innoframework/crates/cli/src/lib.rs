//! Library surface for the `inno` CLI (testable helpers).

pub mod commands;
pub mod errors;

/// Build target for `inno build`.
#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum BuildTarget {
    Web,
    Desktop,
    Mobile,
}
