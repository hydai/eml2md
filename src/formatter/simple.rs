//! Simple markdown formatters

use base64::{engine::general_purpose::STANDARD, Engine};

use crate::eml::{Body, Email, Header, User};

use super::Formatter;

/// Simple formatter that outputs text/plain content
pub struct SimpleFormatter;

impl SimpleFormatter {
    fn format_mail_addr(user: &User) -> String {
        if user.name.is_empty() {
            format!("<{}>", user.email)
        } else {
            format!("{} <{}>", user.name, user.email)
        }
    }

    fn format_users(users: &[User]) -> String {
        users
            .iter()
            .map(Self::format_mail_addr)
            .collect::<Vec<_>>()
            .join("<br>")
    }

    fn replace_attachments(content: &str, email: &Email) -> String {
        let mut result = content.to_string();

        for attachment in &email.attachments {
            if let Some(ref ct) = attachment.content_type {
                if ct.main_type == "image" {
                    if let Some(name) = ct.parameters.get("name") {
                        let placeholder = format!("[image: {}]", name);
                        let base64_data = STANDARD.encode(&attachment.raw);
                        let data_uri = format!(
                            "![{}](data:{};base64,{})",
                            name,
                            ct.mime_type(),
                            base64_data
                        );
                        result = result.replace(&placeholder, &data_uri);
                    }
                }
            }
        }

        result
    }

    fn strip_content(content: &str) -> String {
        content
            .replace("\r\n\r\n", "\n")
            .replace("\r\n", "\n")
    }
}

impl Formatter for SimpleFormatter {
    fn format_header(&self, header: &Header) -> String {
        let date_str = header
            .date
            .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default();

        [
            "|||",
            "|---|---|",
            &format!("|From|{}|", Self::format_mail_addr(&header.from)),
            &format!("|To|{}|", Self::format_users(&header.to)),
            &format!("|CC|{}|", Self::format_users(&header.cc)),
            &format!("|Date|{}|", date_str),
            &format!("|Subject|{}|", header.subject),
        ]
        .join("\n")
    }

    fn format_body(&self, body: &Body, email: &Email) -> String {
        let content = Self::replace_attachments(&body.content, email);
        Self::strip_content(&content)
    }

    fn is_supported_content(&self, body: &Body) -> bool {
        body.content_type == "text/plain"
    }
}

/// HTML-aware formatter (currently same behavior as SimpleFormatter)
pub struct SimpleHtmlFormatter;

impl Formatter for SimpleHtmlFormatter {
    fn format_header(&self, header: &Header) -> String {
        SimpleFormatter.format_header(header)
    }

    fn format_body(&self, body: &Body, email: &Email) -> String {
        SimpleFormatter.format_body(body, email)
    }

    fn is_supported_content(&self, body: &Body) -> bool {
        // Currently same as SimpleFormatter per Python implementation
        body.content_type == "text/plain"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eml::{Email, Header, User};
    use chrono::{FixedOffset, TimeZone};

    fn create_test_email() -> Email {
        Email {
            header: Header {
                from: User::new("Sender", "sender@example.com"),
                to: vec![User::new("Recipient", "recipient@example.com")],
                cc: vec![],
                subject: "Test Subject".to_string(),
                date: Some(
                    FixedOffset::east_opt(0)
                        .unwrap()
                        .with_ymd_and_hms(2024, 1, 1, 12, 0, 0)
                        .unwrap(),
                ),
            },
            body: vec![Body {
                content: "Hello, World!".to_string(),
                content_type: "text/plain".to_string(),
            }],
            attachments: vec![],
        }
    }

    #[test]
    fn test_format_header() {
        let email = create_test_email();
        let formatter = SimpleFormatter;
        let header = formatter.format_header(&email.header);

        assert!(header.contains("|From|Sender <sender@example.com>|"));
        assert!(header.contains("|Subject|Test Subject|"));
        assert!(header.contains("|Date|2024-01-01 12:00:00|"));
    }

    #[test]
    fn test_format_body() {
        let email = create_test_email();
        let formatter = SimpleFormatter;
        let body = formatter.format_body(&email.body[0], &email);

        assert_eq!(body, "Hello, World!");
    }

    #[test]
    fn test_format_full_email() {
        let email = create_test_email();
        let formatter = SimpleFormatter;
        let output = formatter.format(&email);

        assert!(output.contains("|||"));
        assert!(output.contains("Hello, World!"));
    }
}
