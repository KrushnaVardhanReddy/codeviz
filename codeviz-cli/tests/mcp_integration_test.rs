use std::process::{Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use serde_json::Value;

fn send_request(stdin: &mut std::process::ChildStdin, stdout_reader: &mut BufReader<std::process::ChildStdout>, req: Value) -> Value {
    let req_str = serde_json::to_string(&req).unwrap();
    writeln!(stdin, "{}", req_str).unwrap();

    let mut response_str = String::new();
    stdout_reader.read_line(&mut response_str).unwrap();

    serde_json::from_str(&response_str).unwrap()
}

#[test]
fn test_mcp_integration() {
    let mut child = Command::new("cargo")
        .args(&["run", "-p", "codeviz-cli", "--", "serve", "--mcp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn MCP server");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let mut stdout_reader = BufReader::new(stdout);

    // Test tools/list
    let req_list = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    });
    let res_list = send_request(&mut stdin, &mut stdout_reader, req_list);
    assert_eq!(res_list["jsonrpc"], "2.0");
    assert_eq!(res_list["id"], 1);

    let tools = res_list["result"]["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 6);

    let required_tools = vec!["get_module_graph", "get_callers", "get_callees", "get_class_hierarchy", "find_entry_points", "explain_path"];

    for tool in required_tools {
        let req_tool = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": tool,
                "arguments": {
                    "path": "src",
                    "fn_name": "main",
                    "from": "a",
                    "to": "b"
                }
            }
        });
        let res_tool = send_request(&mut stdin, &mut stdout_reader, req_tool);
        assert!(res_tool["error"].is_null(), "Tool {} returned error: {:?}", tool, res_tool["error"]);
        assert!(!res_tool["result"].is_null(), "Tool {} returned no result", tool);
    }


    // Test unknown tool
    let req_unknown = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "unknown_tool",
            "arguments": {}
        }
    });
    let res_unknown = send_request(&mut stdin, &mut stdout_reader, req_unknown);
    assert_eq!(res_unknown["error"]["code"], -32601);

    // Test invalid params
    let req_invalid = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/call",
        "params": {
            "name": "get_module_graph",
            "arguments": {} // missing path
        }
    });
    let res_invalid = send_request(&mut stdin, &mut stdout_reader, req_invalid);
    assert_eq!(res_invalid["error"]["code"], -32602);

    drop(stdin);
    let _ = child.wait();
}
