use codeviz_mcp::server::handle_request;
use codeviz_mcp::tools::JsonRpcRequest;
use serde_json::json;

#[test]
fn test_unknown_tool() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "fake_tool",
            "arguments": {}
        })),
    };

    let registry = codeviz_core::parser::LanguageRegistry::new();
    let res = handle_request(req, &registry);
    assert_eq!(res.error.unwrap().code, -32601);
}

#[test]
fn test_missing_params() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/call".to_string(),
        params: None,
    };

    let registry = codeviz_core::parser::LanguageRegistry::new();
    let res = handle_request(req, &registry);
    assert_eq!(res.error.unwrap().code, -32601);
}

#[test]
fn test_tools_list() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(3)),
        method: "tools/list".to_string(),
        params: None,
    };

    let registry = codeviz_core::parser::LanguageRegistry::new();
    let res = handle_request(req, &registry);
    assert!(res.result.is_some());
    let result = res.result.unwrap();
    let tools = result.get("tools").unwrap().as_array().unwrap();
    assert_eq!(tools.len(), 6);
}

#[test]
fn test_invalid_tool_params() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(4)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "get_module_graph",
            "arguments": {} // Missing required 'path'
        })),
    };

    let registry = codeviz_core::parser::LanguageRegistry::new();
    let res = handle_request(req, &registry);
    assert_eq!(res.error.unwrap().code, -32602);
}
