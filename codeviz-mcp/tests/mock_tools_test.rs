use codeviz_mcp::server::handle_request;
use codeviz_mcp::tools::JsonRpcRequest;
use serde_json::json;

#[test]
fn test_all_6_tools_success() {
    let tools = vec![
        ("get_module_graph", json!({"path": "src"})),
        ("get_callers", json!({"fn_name": "main", "path": "src"})),
        ("get_callees", json!({"fn_name": "main", "path": "src"})),
        ("get_class_hierarchy", json!({"path": "src"})),
        ("find_entry_points", json!({"path": "src"})),
        (
            "explain_path",
            json!({"from": "a", "to": "b", "path": "src"}),
        ),
    ];

    for (i, (tool, args)) in tools.into_iter().enumerate() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(i)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": tool,
                "arguments": args
            })),
        };

        let registry = codeviz_core::parser::LanguageRegistry::new();
        let res = handle_request(req, &registry);
        assert!(res.error.is_none(), "Tool {} failed", tool);
        assert!(res.result.is_some(), "Tool {} had no result", tool);
    }
}
