//! Content-Type header parsing utilities

use std::collections::HashMap;

/// Parsed MIME content type
#[derive(Debug, Clone, PartialEq)]
pub struct ContentType {
    /// Main type (e.g., "image", "text")
    pub main_type: String,
    /// Sub type (e.g., "png", "plain")
    pub sub_type: String,
    /// Parameters (e.g., name="image.png")
    pub parameters: HashMap<String, String>,
}

impl ContentType {
    /// Parse a Content-Type header string
    ///
    /// # Example
    /// ```
    /// use eml2md::content_type::ContentType;
    ///
    /// let ct = ContentType::parse("image/png; name=\"test.png\"");
    /// assert_eq!(ct.main_type, "image");
    /// assert_eq!(ct.sub_type, "png");
    /// assert_eq!(ct.parameters.get("name"), Some(&"test.png".to_string()));
    /// ```
    pub fn parse(content_type: &str) -> Self {
        let mut parameters = HashMap::new();
        let tokens: Vec<&str> = content_type.split(';').map(|s| s.trim()).collect();

        let (main_type, sub_type) = if let Some(type_part) = tokens.first() {
            let parts: Vec<&str> = type_part.split('/').collect();
            if parts.len() == 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (type_part.to_string(), String::new())
            }
        } else {
            (String::new(), String::new())
        };

        for token in tokens.iter().skip(1) {
            if let Some((key, val)) = token.split_once('=') {
                let val = val.trim_matches('"').to_string();
                parameters.insert(key.trim().to_string(), val);
            }
        }

        ContentType {
            main_type,
            sub_type,
            parameters,
        }
    }

    /// Get the full MIME type string (e.g., "image/png")
    pub fn mime_type(&self) -> String {
        format!("{}/{}", self.main_type, self.sub_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let ct = ContentType::parse("text/plain");
        assert_eq!(ct.main_type, "text");
        assert_eq!(ct.sub_type, "plain");
        assert!(ct.parameters.is_empty());
    }

    #[test]
    fn test_parse_with_parameters() {
        let ct = ContentType::parse("image/png; name=\"test.png\"; charset=utf-8");
        assert_eq!(ct.main_type, "image");
        assert_eq!(ct.sub_type, "png");
        assert_eq!(ct.parameters.get("name"), Some(&"test.png".to_string()));
        assert_eq!(ct.parameters.get("charset"), Some(&"utf-8".to_string()));
    }
}
