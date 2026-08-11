/// Normalize a file path to always use forward slashes.
/// This is necessary for cross-platform compatibility on Windows.
pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_slash_unchanged() {
        assert_eq!(normalize_path("src/main.rs"), "src/main.rs");
    }

    #[test]
    fn test_backslash_converted() {
        assert_eq!(normalize_path("src\\main.rs"), "src/main.rs");
    }

    #[test]
    fn test_mixed_slashes() {
        assert_eq!(normalize_path("src\\lib\\utils.rs"), "src/lib/utils.rs");
    }
}
