use codeviz_core::DummyNode;

/// Starts the dummy MCP server.
/// Returns a Result to signify success or error.
pub fn start_mcp_server() -> Result<(), String> {
    let _node = DummyNode::new("mcp-node".to_string())?;
    // Stub implementation to return success for now
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_mcp_server() {
        let result = start_mcp_server();
        assert!(result.is_ok());
    }
}
