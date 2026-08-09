use codeviz_mcp::{JsonRpcRequest, handle_request};
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

        let res = handle_request(req);
        assert!(res.error.is_none(), "Tool {} failed", tool);
        assert!(res.result.is_some(), "Tool {} had no result", tool);
    }
}
