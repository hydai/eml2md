//! eml2md - Convert EML files to Markdown
//!
//! This crate provides functionality to parse EML (email) files
//! and convert them to Markdown format.

pub mod content_type;
pub mod eml;
pub mod formatter;

pub use eml::{parse_eml, Email};
pub use formatter::{create_formatter, Formatter};
