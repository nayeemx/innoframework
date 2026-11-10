//! Output sanitization (TR-5.3, AI-9).
//!
//! Sanitizes model output before it is rendered to the UI. Removes dangerous
//! tags entirely and escapes all HTML special characters so output is rendered
//! as text, not markup.

use crate::error::{ErrorCode, InnoframeworkError};

pub const SANITIZER_ERROR: ErrorCode = ErrorCode::new_const(1800);

// Tags whose entire element (opening + content + closing) is removed.
const DANGEROUS_TAGS: &[&str] = &["script", "iframe", "object", "embed", "applet"];

/// Sanitize `html` for safe insertion into the UI.
///
/// - Removes `<script>...</script>`, `<iframe>...</iframe>`, etc. entirely.
/// - Escapes `& < > " '` in all remaining text so markup renders as text.
///
/// Returns `Err(SANITIZER_ERROR)` only on catastrophic failure.
pub fn sanitize(html: &str) -> Result<String, InnoframeworkError> {
    let mut out = String::with_capacity(html.len());
    let mut i = 0;

    while i < html.len() {
        if html[i..].starts_with('<') {
            // Check if this is a dangerous tag (opening or closing).
            if let Some(skip) = skip_dangerous_tag(&html[i..]) {
                i += skip;
                continue;
            }
            // Not dangerous — escape the '<' and continue; the rest of the
            // tag will be escaped character by character below.
            escape_char('<', &mut out);
            i += 1;
        } else {
            escape_char(html.chars().nth(i).unwrap(), &mut out);
            i += 1;
        }
    }

    Ok(out)
}

/// If `s` starts with a dangerous tag, return the number of bytes to skip
/// (including the tag's content up to and including its closing tag).
/// Otherwise returns None.
fn skip_dangerous_tag(s: &str) -> Option<usize> {
    for tag in DANGEROUS_TAGS {
        let lower = s.to_lowercase();
        // Match opening tag: <tag, </tag, <tag ...
        if lower.starts_with(&format!("<{}", tag)) || lower.starts_with(&format!("</{}", tag)) {
            // Find the closing tag.
            let close_tag = format!("</{}>", tag);
            if let Some(close_pos) = lower.find(&close_tag.to_lowercase()) {
                return Some(close_pos + close_tag.len());
            }
            // No closing tag found — skip the rest of the input.
            return Some(s.len());
        }
    }
    None
}

/// Escape a single character for safe text rendering.
fn escape_char(c: char, out: &mut String) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        '\'' => out.push_str("&#39;"),
        _ => out.push(c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_passes_through() {
        assert_eq!(sanitize("Hello, world!").unwrap(), "Hello, world!");
    }

    #[test]
    fn html_entities_are_escaped() {
        assert_eq!(sanitize("<b>bold</b>").unwrap(), "&lt;b&gt;bold&lt;/b&gt;");
        assert_eq!(sanitize("a & b").unwrap(), "a &amp; b");
    }

    #[test]
    fn script_tags_are_removed() {
        let html = "Before <script>alert('xss')</script> After";
        let result = sanitize(html).unwrap();
        assert!(!result.contains("<script"));
        assert!(!result.contains("alert"));
        assert!(result.contains("Before"));
        assert!(result.contains("After"));
    }

    #[test]
    fn iframe_is_removed() {
        let html = "<iframe src=\"evil.com\"></iframe>";
        let result = sanitize(html).unwrap();
        assert!(!result.contains("iframe"));
    }

    #[test]
    fn multiple_script_tags() {
        let html = "<script>a</script><script>b</script>";
        let result = sanitize(html).unwrap();
        assert!(!result.contains("script"));
    }

    #[test]
    fn dangerous_tag_without_closing_is_skipped() {
        let html = "<script>alert('xss')";
        let result = sanitize(html).unwrap();
        assert!(!result.contains("script"));
        assert!(!result.contains("alert"));
    }

    #[test]
    fn non_dangerous_tags_are_escaped_not_rendered() {
        let html = "<b>bold</b>";
        let result = sanitize(html).unwrap();
        assert_eq!(result, "&lt;b&gt;bold&lt;/b&gt;");
    }

    #[test]
    fn attributes_are_escaped() {
        let html = r#"<a href="javascript:alert(1)">Link</a>"#;
        let result = sanitize(html).unwrap();
        // Everything is escaped, so javascript: is rendered as inert text.
        assert!(result.contains("&quot;javascript:"));
        assert!(result.contains("Link"));
        assert!(!result.contains("<a"));
    }

    #[test]
    fn object_and_embed_removed() {
        let html = "<object data=\"evil.swf\"></object><embed src=\"evil.swf\">";
        let result = sanitize(html).unwrap();
        assert!(!result.contains("object"));
        assert!(!result.contains("embed"));
    }
}
