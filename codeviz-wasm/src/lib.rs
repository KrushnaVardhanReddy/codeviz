use codeviz_core::DummyNode;

/// A dummy WASM function that parses source code.
/// Since we don't have wasm-bindgen yet, this is a plain Rust function.
pub fn dummy_parse(source: &str) -> Result<String, String> {
    if source.is_empty() {
        return Err("Source cannot be empty".to_string());
    }

    let node = DummyNode::new("wasm-node".to_string())?;
    Ok(format!("Parsed node: {}", node.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_parse() {
        let result = dummy_parse("let x = 1;");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Parsed node: wasm-node");
    }

    #[test]
    fn test_dummy_parse_empty() {
        let result = dummy_parse("");
        assert!(result.is_err());
    }
}
