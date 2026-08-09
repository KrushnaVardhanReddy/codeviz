use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::io::{self, Write};

/// Starts the MCP server over stdio.
/// Returns a Result to signify success or error.
pub fn start_mcp_server() -> Result<(), String> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let deserializer = serde_json::Deserializer::from_reader(stdin.lock());
    let iterator = deserializer.into_iter::<JsonRpcRequest>();

    for req in iterator {
        let req = match req {
            Ok(r) => r,
            Err(_) => {
                let err_res = error_response(None, JsonRpcError::invalid_params("Invalid JSON"));
                let out = serde_json::to_string(&err_res).map_err(|e| e.to_string())?;
                writeln!(stdout, "{}", out).map_err(|e| e.to_string())?;
                continue;
            }
        };

        let res = handle_request(req);
        let out = serde_json::to_string(&res).map_err(|e| e.to_string())?;
        writeln!(stdout, "{}", out).map_err(|e| e.to_string())?;
        stdout.flush().map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Represents a JSON-RPC 2.0 request.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JsonRpcRequest {
    /// The JSON-RPC version, typically "2.0".
    pub jsonrpc: String,
    /// The request ID.
    pub id: Option<Value>,
    /// The method being called.
    pub method: String,
    /// The parameters for the method.
    pub params: Option<Value>,
}

/// Represents a JSON-RPC 2.0 response.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcResponse {
    /// The JSON-RPC version, typically "2.0".
    pub jsonrpc: String,
    /// The request ID.
    pub id: Option<Value>,
    /// The result of the method call, if successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// The error object, if the call failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// Represents a JSON-RPC 2.0 error object.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcError {
    /// The error code.
    pub code: i32,
    /// A short description of the error.
    pub message: String,
}

impl JsonRpcError {
    /// Creates a new error for an unknown tool.
    pub fn unknown_tool(tool_name: &str) -> Self {
        Self {
            code: -32601,
            message: format!("Unknown tool: {}", tool_name),
        }
    }

    /// Creates a new error for invalid parameters.
    pub fn invalid_params(details: &str) -> Self {
        Self {
            code: -32602,
            message: format!("Invalid params: {}", details),
        }
    }

    /// Creates a new error for an internal server error.
    pub fn internal_error(details: &str) -> Self {
        Self {
            code: -32603,
            message: format!("Internal error: {}", details),
        }
    }
}

/// Handles an incoming JSON-RPC request and dispatches it to the appropriate handler.
pub fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    if req.method == "tools/list" {
        return handle_tools_list(req.id);
    } else if req.method == "tools/call" {
        return handle_tools_call(req);
    }

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id,
        result: None,
        error: Some(JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
        }),
    }
}

/// Handles the `tools/list` JSON-RPC method, returning definitions for all available tools.
pub fn handle_tools_list(id: Option<Value>) -> JsonRpcResponse {
    let tools = serde_json::json!([
        {
            "name": "get_module_graph",
            "description": "Returns the full module dependency graph for a directory.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "max_nodes": { "type": "number" }
                },
                "required": ["path"]
            }
        },
        {
            "name": "get_callers",
            "description": "Returns all nodes that call a given function.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fn_name": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["fn_name", "path"]
            }
        },
        {
            "name": "get_callees",
            "description": "Returns all nodes called by a given function.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fn_name": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["fn_name", "path"]
            }
        },
        {
            "name": "get_class_hierarchy",
            "description": "Returns the full inheritance tree as a Mermaid classDiagram.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }
        },
        {
            "name": "find_entry_points",
            "description": "Returns all nodes with no incoming Calls edges.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }
        },
        {
            "name": "explain_path",
            "description": "Returns the shortest dependency path between two named nodes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["from", "to", "path"]
            }
        }
    ]);

    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(serde_json::json!({ "tools": tools })),
        error: None,
    }
}

/// Handles the `tools/call` JSON-RPC method, dispatching to specific tool handlers based on the requested tool name.
pub fn handle_tools_call(req: JsonRpcRequest) -> JsonRpcResponse {
    let id = req.id.clone();

    let params = match req.params {
        Some(p) => p,
        None => return error_response(id, JsonRpcError::invalid_params("Missing params")),
    };

    let tool_name = match params.get("name").and_then(|n| n.as_str()) {
        Some(name) => name,
        None => return error_response(id, JsonRpcError::invalid_params("Missing tool name")),
    };

    let tool_args = params.get("arguments").unwrap_or(&Value::Null);

    // Mock CodeGraph logic
    use codeviz_core::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};

    let mock_meta = GraphMeta {
        language: "mock".to_string(),
        source_root: "/".to_string(),
        generated_at: "".to_string(),
        node_count: 2,
        edge_count: 1,
    };
    let mut mock_graph = CodeGraph::new(mock_meta.clone());
    let mock_node1 = Node {
        id: "a".to_string(),
        label: "a".to_string(),
        kind: NodeKind::Function { is_async: false },
        file_path: "a.rs".to_string(),
        line: None,
        is_public: true,
    };
    let mock_node2 = Node {
        id: "b".to_string(),
        label: "b".to_string(),
        kind: NodeKind::Function { is_async: false },
        file_path: "a.rs".to_string(),
        line: None,
        is_public: true,
    };
    mock_graph.nodes.push(mock_node1.clone());
    mock_graph.nodes.push(mock_node2.clone());
    mock_graph.edges.push(Edge {
        from_id: "a".to_string(),
        to_id: "b".to_string(),
        kind: EdgeKind::Calls,
    });

    match tool_name {
        "get_module_graph" => {
            if !tool_args.is_object() || tool_args.get("path").is_none() {
                return error_response(id, JsonRpcError::invalid_params("Missing path"));
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "graph": mock_graph,
                    "mermaid": "graph TD\n  a --> b"
                })),
                error: None,
            }
        }
        "get_callers" => {
            if !tool_args.is_object()
                || tool_args.get("fn_name").is_none()
                || tool_args.get("path").is_none()
            {
                return error_response(id, JsonRpcError::invalid_params("Missing fn_name or path"));
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "callers": [mock_node1]
                })),
                error: None,
            }
        }
        "get_callees" => {
            if !tool_args.is_object()
                || tool_args.get("fn_name").is_none()
                || tool_args.get("path").is_none()
            {
                return error_response(id, JsonRpcError::invalid_params("Missing fn_name or path"));
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "callees": [mock_node2]
                })),
                error: None,
            }
        }
        "get_class_hierarchy" => {
            if !tool_args.is_object() || tool_args.get("path").is_none() {
                return error_response(id, JsonRpcError::invalid_params("Missing path"));
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "mermaid": "classDiagram\n"
                })),
                error: None,
            }
        }
        "find_entry_points" => {
            if !tool_args.is_object() || tool_args.get("path").is_none() {
                return error_response(id, JsonRpcError::invalid_params("Missing path"));
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "nodes": [mock_node1]
                })),
                error: None,
            }
        }
        "explain_path" => {
            if !tool_args.is_object()
                || tool_args.get("from").is_none()
                || tool_args.get("to").is_none()
                || tool_args.get("path").is_none()
            {
                return error_response(
                    id,
                    JsonRpcError::invalid_params("Missing from, to, or path"),
                );
            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "nodes": [mock_node1, mock_node2],
                    "exists": true
                })),
                error: None,
            }
        }
        _ => error_response(id, JsonRpcError::unknown_tool(tool_name)),
    }
}

/// Helper function to create an error response.
pub fn error_response(id: Option<Value>, error: JsonRpcError) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(error),
    }
}
