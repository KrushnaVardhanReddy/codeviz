use std::error::Error;
use std::fmt;

/// Errors that can occur during markdown injection.
#[derive(Debug, PartialEq, Eq)]
pub enum InjectError {
    /// Neither sentinel tag found in the markdown string.
    MissingTags,
    /// One of the tags is missing, they are in the wrong order, or there are multiple of the same tag.
    MalformedTags,
}

impl fmt::Display for InjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InjectError::MissingTags => write!(f, "Missing CODEVIZ sentinel tags in markdown"),
            InjectError::MalformedTags => write!(
                f,
                "Malformed CODEVIZ sentinel tags (missing one or wrong order)"
            ),
        }
    }
}

impl Error for InjectError {}

const TAG_START: &str = "<!-- CODEVIZ_START -->";
const TAG_END: &str = "<!-- CODEVIZ_END -->";

/// Injects mermaid content between the CODEVIZ_START and CODEVIZ_END tags.
/// All content before and after the tags is preserved byte-for-byte.
///
/// # Arguments
/// * `markdown` - The original markdown content.
/// * `mermaid` - The mermaid content to inject.
///
/// # Returns
/// The modified markdown string on success, or an `InjectError` if tags are missing or malformed.
pub fn inject_mermaid(markdown: &str, mermaid: &str) -> Result<String, InjectError> {
    let start_indices: Vec<_> = markdown.match_indices(TAG_START).collect();
    let end_indices: Vec<_> = markdown.match_indices(TAG_END).collect();

    if start_indices.is_empty() && end_indices.is_empty() {
        return Err(InjectError::MissingTags);
    }

    if start_indices.len() != 1 || end_indices.len() != 1 {
        return Err(InjectError::MalformedTags);
    }

    let (start_idx, _) = start_indices[0];
    let (end_idx, _) = end_indices[0];

    if start_idx >= end_idx {
        return Err(InjectError::MalformedTags);
    }

    let before = &markdown[..start_idx + TAG_START.len()];
    let after = &markdown[end_idx..];

    let mut injected = String::with_capacity(before.len() + mermaid.len() + after.len() + 32);
    injected.push_str(before);
    injected.push_str("\n```mermaid\n");
    if !mermaid.is_empty() {
        injected.push_str(mermaid);
        if !mermaid.ends_with('\n') {
            injected.push('\n');
        }
    }
    injected.push_str("```\n");
    injected.push_str(after);

    Ok(injected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_injection() {
        let markdown = "Before\n<!-- CODEVIZ_START -->\nold content\n<!-- CODEVIZ_END -->\nAfter";
        let mermaid = "graph TD\n    A --> B";

        let result = inject_mermaid(markdown, mermaid).unwrap();

        assert_eq!(
            result,
            "Before\n<!-- CODEVIZ_START -->\n```mermaid\ngraph TD\n    A --> B\n```\n<!-- CODEVIZ_END -->\nAfter"
        );
    }

    #[test]
    fn test_missing_both_tags() {
        let markdown = "Just some normal markdown here.";
        let mermaid = "graph TD\n    A --> B";

        let result = inject_mermaid(markdown, mermaid);
        assert_eq!(result, Err(InjectError::MissingTags));
    }

    #[test]
    fn test_only_one_tag() {
        let markdown_start_only = "<!-- CODEVIZ_START -->\nSome content";
        let mermaid = "graph TD\n    A --> B";
        let result = inject_mermaid(markdown_start_only, mermaid);
        assert_eq!(result, Err(InjectError::MalformedTags));

        let markdown_end_only = "Some content\n<!-- CODEVIZ_END -->";
        let result = inject_mermaid(markdown_end_only, mermaid);
        assert_eq!(result, Err(InjectError::MalformedTags));

        let out_of_order = "<!-- CODEVIZ_END -->\n<!-- CODEVIZ_START -->";
        let result = inject_mermaid(out_of_order, mermaid);
        assert_eq!(result, Err(InjectError::MalformedTags));

        let duplicate_tags = "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->";
        let result = inject_mermaid(duplicate_tags, mermaid);
        assert_eq!(result, Err(InjectError::MalformedTags));
    }

    #[test]
    fn test_idempotency() {
        let original_markdown = "Header\n<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->\nFooter";
        let mermaid = "graph TD\n    C --> D";

        let first_injection = inject_mermaid(original_markdown, mermaid).unwrap();
        let second_injection = inject_mermaid(&first_injection, mermaid).unwrap();

        assert_eq!(first_injection, second_injection);
    }

    #[test]
    fn test_empty_mermaid_string() {
        let markdown = "Header\n<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->";
        let mermaid = "";

        let result = inject_mermaid(markdown, mermaid).unwrap();

        assert_eq!(
            result,
            "Header\n<!-- CODEVIZ_START -->\n```mermaid\n```\n<!-- CODEVIZ_END -->"
        );
    }
}
