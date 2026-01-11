//! Markdown formatters for email content

mod simple;

use crate::eml::{Body, Email, Header};

pub use simple::{SimpleFormatter, SimpleHtmlFormatter};

/// Trait for formatting email to markdown
pub trait Formatter {
    /// Format an entire email to markdown
    fn format(&self, email: &Email) -> String {
        let header = self.format_header(&email.header);
        let body = self.format_bodies(&email.body, email);
        format!("{}\n\n{}", header, body)
    }

    /// Format email header as markdown table
    fn format_header(&self, header: &Header) -> String;

    /// Format all bodies
    fn format_bodies(&self, bodies: &[Body], email: &Email) -> String {
        bodies
            .iter()
            .filter(|body| self.is_supported_content(body))
            .map(|body| self.format_body(body, email))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Format a single body
    fn format_body(&self, body: &Body, email: &Email) -> String;

    /// Check if content type is supported
    fn is_supported_content(&self, body: &Body) -> bool;
}

/// Create a formatter by name
pub fn create_formatter(name: &str) -> Box<dyn Formatter> {
    match name {
        "html" => Box::new(SimpleHtmlFormatter),
        _ => Box::new(SimpleFormatter),
    }
}

/// Format an email to markdown using the specified formatter
pub fn format_markdown(email: &Email, formatter_name: &str) -> String {
    let formatter = create_formatter(formatter_name);
    formatter.format(email)
}
