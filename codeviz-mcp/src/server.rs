use crate::tools::*;
use codeviz_core::LanguageRegistry;
use codeviz_go::GoParser;
use codeviz_python::PythonParser;
use codeviz_typescript::TypeScriptParser;
use serde_json::{Value, json};
use std::io::{self, BufRead, Write};

/// Starts the MCP JSON-RPC server over stdio.
pub fn start_mcp_server() -> Result<(), String> {
    let mut registry = LanguageRegistry::new();
    registry.register(Box::new(PythonParser::new()));
    registry.register(Box::new(TypeScriptParser::new()));
    registry.register(Box::new(GoParser::new()));

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let request: Result<JsonRpcRequest, _> = serde_json::from_str(trimmed);
                let response = match request {
                    Ok(req) => handle_request(req, &registry),
                    Err(_) => error_response(
                        None,
                        JsonRpcError {
                            code: ERROR_PARSE,
                            message: "Parse error".to_string(),
                            data: None,
                        },
                    ),
                };

                let response_json =
                    serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());
                if let Err(e) = writeln!(writer, "{}", response_json) {
                    eprintln!("Failed to write to stdout: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stdin: {}", e);
                break;
            }
        }
    }

    Ok(())
}

pub fn handle_request(req: JsonRpcRequest, registry: &LanguageRegistry) -> JsonRpcResponse {
    let id = req.id;
    let method = req.method.as_str();

    match method {
        "tools/list" => {
            let tools = list_tools();
            success_response(id, json!({ "tools": tools }))
        }
        "tools/call" => {
            let params = req.params.unwrap_or(Value::Null);
            let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let tool_args = params.get("arguments").cloned().unwrap_or(Value::Null);

            let result = match tool_name {
                "get_module_graph" => handle_get_module_graph(tool_args, registry),
                "get_callers" => handle_get_callers(tool_args, registry),
                "get_callees" => handle_get_callees(tool_args, registry),
                "get_class_hierarchy" => handle_get_class_hierarchy(tool_args, registry),
                "find_entry_points" => handle_find_entry_points(tool_args, registry),
                "explain_path" => handle_explain_path(tool_args, registry),
                _ => return error_response(id, JsonRpcError::method_not_found(tool_name)),
            };

            match result {
                Ok(val) => success_response(id, val),
                Err(err) => error_response(id, err),
            }
        }
        _ => error_response(id, JsonRpcError::method_not_found(method)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_unknown_method() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(1)),
            method: "unknown/method".to_string(),
            params: None,
        };
        let reg = LanguageRegistry::new();
        let res = handle_request(req, &reg);
        let err = res.error.unwrap();
        assert_eq!(err.code, ERROR_METHOD_NOT_FOUND);
    }

    #[test]
    fn test_unknown_tool() {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(json!(2)),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "unknown_tool",
                "arguments": {}
            })),
        };
        let reg = LanguageRegistry::new();
        let res = handle_request(req, &reg);
        let err = res.error.unwrap();
        assert_eq!(err.code, ERROR_METHOD_NOT_FOUND);
    }
}
