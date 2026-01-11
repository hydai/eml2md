//! EML file parsing module

use std::path::Path;

use anyhow::{Context, Result};
use chrono::{DateTime, FixedOffset};
use mail_parser::{MessageParser, MimeHeaders};

use crate::content_type::ContentType;

/// Represents an email address with name and address
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

/// Email header information
#[derive(Debug, Clone)]
pub struct Header {
    pub from: User,
    pub to: Vec<User>,
    pub cc: Vec<User>,
    pub subject: String,
    pub date: Option<DateTime<FixedOffset>>,
}

/// Email body content
#[derive(Debug, Clone)]
pub struct Body {
    pub content: String,
    pub content_type: String,
}

/// Email attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    pub raw: Vec<u8>,
    pub content_type: Option<ContentType>,
    pub content_id: Option<String>,
}

/// Parsed email structure
#[derive(Debug, Clone)]
pub struct Email {
    pub header: Header,
    pub body: Vec<Body>,
    pub attachments: Vec<Attachment>,
}

/// Parse an EML file into an Email structure
pub fn parse_eml(path: &Path) -> Result<Email> {
    let content =
        std::fs::read(path).with_context(|| format!("Failed to read file: {}", path.display()))?;

    parse_eml_bytes(&content)
}

/// Parse EML content from bytes
pub fn parse_eml_bytes(content: &[u8]) -> Result<Email> {
    let message = MessageParser::default()
        .parse(content)
        .context("Failed to parse email message")?;

    // Parse From header
    let from = message
        .from()
        .and_then(|addrs| addrs.first())
        .map(|addr| {
            User::new(
                addr.name().unwrap_or_default(),
                addr.address().unwrap_or_default(),
            )
        })
        .unwrap_or_else(|| User::new("", ""));

    // Parse To header
    let to: Vec<User> = message
        .to()
        .map(|addrs| {
            addrs
                .iter()
                .map(|addr| {
                    User::new(
                        addr.name().unwrap_or_default(),
                        addr.address().unwrap_or_default(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // Parse CC header
    let cc: Vec<User> = message
        .cc()
        .map(|addrs| {
            addrs
                .iter()
                .map(|addr| {
                    User::new(
                        addr.name().unwrap_or_default(),
                        addr.address().unwrap_or_default(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // Parse subject
    let subject = message.subject().unwrap_or_default().to_string();

    // Parse date
    let date = message.date().and_then(|dt| {
        DateTime::from_timestamp(dt.to_timestamp(), 0)
            .map(|utc| utc.with_timezone(&FixedOffset::east_opt(0).unwrap()))
    });

    let header = Header {
        from,
        to,
        cc,
        subject,
        date,
    };

    // Parse body parts
    let mut body = Vec::new();
    let mut attachments = Vec::new();

    // Get text body
    if let Some(text) = message.body_text(0) {
        body.push(Body {
            content: text.to_string(),
            content_type: "text/plain".to_string(),
        });
    }

    // Get HTML body
    if let Some(html) = message.body_html(0) {
        body.push(Body {
            content: html.to_string(),
            content_type: "text/html".to_string(),
        });
    }

    // Parse attachments
    for attachment in message.attachments() {
        let content_type = attachment
            .content_type()
            .map(|ct: &mail_parser::ContentType| {
                let type_str = format!("{}/{}", ct.ctype(), ct.subtype().unwrap_or_default());
                ContentType::parse(&type_str)
            });

        attachments.push(Attachment {
            raw: attachment.contents().to_vec(),
            content_type,
            content_id: None,
        });
    }

    Ok(Email {
        header,
        body,
        attachments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_email() {
        let eml = b"From: sender@example.com\r\n\
                    To: recipient@example.com\r\n\
                    Subject: Test Subject\r\n\
                    Date: Mon, 1 Jan 2024 12:00:00 +0000\r\n\
                    Content-Type: text/plain\r\n\r\n\
                    Hello, World!";

        let email = parse_eml_bytes(eml).unwrap();
        assert_eq!(email.header.subject, "Test Subject");
        assert_eq!(email.header.from.email, "sender@example.com");
        assert!(
            !email.body.is_empty(),
            "Email should have at least one body"
        );
        assert!(
            email.body.iter().any(|b| b.content_type == "text/plain"),
            "Email should contain text/plain body"
        );
    }
}
