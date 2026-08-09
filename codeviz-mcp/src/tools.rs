use codeviz_core::render::mermaid::{DiagramKind, MermaidRenderer};
use codeviz_core::{CodeGraph, EdgeKind, GraphMeta, LanguageRegistry, Node, NodeKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Option<Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub const ERROR_PARSE: i32 = -32700;
pub const ERROR_INVALID_REQUEST: i32 = -32600;
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
pub const ERROR_INVALID_PARAMS: i32 = -32602;
pub const ERROR_INTERNAL: i32 = -32603;

impl JsonRpcError {
    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: ERROR_METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }

    pub fn invalid_params(msg: &str) -> Self {
        Self {
            code: ERROR_INVALID_PARAMS,
            message: msg.to_string(),
            data: None,
        }
    }

    pub fn internal_error(msg: &str) -> Self {
        Self {
            code: ERROR_INTERNAL,
            message: msg.to_string(),
            data: None,
        }
    }
}

/// Helper function to create a successful response
pub fn success_response(id: Option<Value>, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}

/// Helper function to create an error response
pub fn error_response(id: Option<Value>, error: JsonRpcError) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(error),
    }
}

/// Defines a tool to be returned in tools/list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Returns the list of tools available in the MCP server.
pub fn list_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "get_module_graph".to_string(),
            description: "Returns the full module dependency graph for a directory.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "max_nodes": { "type": "number", "description": "Optional maximum number of nodes to return" }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "get_callers".to_string(),
            description: "Returns all nodes that call a given function.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fn_name": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["fn_name", "path"]
            }),
        },
        ToolDefinition {
            name: "get_callees".to_string(),
            description: "Returns all nodes called by a given function.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "fn_name": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["fn_name", "path"]
            }),
        },
        ToolDefinition {
            name: "get_class_hierarchy".to_string(),
            description: "Returns the full inheritance tree as a Mermaid classDiagram.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "find_entry_points".to_string(),
            description:
                "Returns all nodes with no incoming Calls edges (i.e., nothing calls them)."
                    .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "explain_path".to_string(),
            description: "Returns the shortest dependency path between two named nodes."
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "path": { "type": "string" }
                },
                "required": ["from", "to", "path"]
            }),
        },
    ]
}

/// Prunes graph for MCP to respect max_nodes.
/// Just a simple truncation.
fn prune_graph(graph: &mut CodeGraph, max_nodes: usize) {
    if graph.nodes.len() > max_nodes {
        graph.nodes.truncate(max_nodes);
        let valid_nodes: HashSet<String> = graph.nodes.iter().map(|n| n.id.clone()).collect();
        graph
            .edges
            .retain(|e| valid_nodes.contains(&e.from_id) && valid_nodes.contains(&e.to_id));
        graph.meta.node_count = graph.nodes.len();
        graph.meta.edge_count = graph.edges.len();
    }
}

/// Handles the `get_module_graph` tool.
pub fn handle_get_module_graph(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;
    let max_nodes = params
        .get("max_nodes")
        .and_then(|v| v.as_u64())
        .unwrap_or(200) as usize;

    let mut graph = build_graph_from_path(path, registry)?;

    let original_len = graph.nodes.len();
    prune_graph(&mut graph, max_nodes);
    let truncated = original_len > max_nodes;

    let mermaid = MermaidRenderer::new().render(&graph, DiagramKind::ModuleGraph);

    Ok(json!({
        "graph": graph,
        "mermaid": mermaid,
        "truncated": truncated
    }))
}

/// Handles the `get_callers` tool.
pub fn handle_get_callers(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;
    let fn_name = params
        .get("fn_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'fn_name' parameter"))?;

    let graph = build_graph_from_path(path, registry)?;

    // Find node with given fn_name
    let target_node = graph
        .nodes
        .iter()
        .find(|n| n.label == fn_name || n.id.ends_with(fn_name));

    if let Some(target) = target_node {
        let caller_ids: HashSet<&String> = graph
            .edges
            .iter()
            .filter(|e| e.to_id == target.id && e.kind == EdgeKind::Calls)
            .map(|e| &e.from_id)
            .collect();

        let callers: Vec<&Node> = graph
            .nodes
            .iter()
            .filter(|n| caller_ids.contains(&n.id))
            .collect();

        Ok(json!({
            "callers": callers
        }))
    } else {
        Ok(json!({ "callers": [] })) // Not found
    }
}

/// Handles the `get_callees` tool.
pub fn handle_get_callees(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;
    let fn_name = params
        .get("fn_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'fn_name' parameter"))?;

    let graph = build_graph_from_path(path, registry)?;

    let target_node = graph
        .nodes
        .iter()
        .find(|n| n.label == fn_name || n.id.ends_with(fn_name));

    if let Some(target) = target_node {
        let callee_ids: HashSet<&String> = graph
            .edges
            .iter()
            .filter(|e| e.from_id == target.id && e.kind == EdgeKind::Calls)
            .map(|e| &e.to_id)
            .collect();

        let callees: Vec<&Node> = graph
            .nodes
            .iter()
            .filter(|n| callee_ids.contains(&n.id))
            .collect();

        Ok(json!({
            "callees": callees
        }))
    } else {
        Ok(json!({ "callees": [] })) // Not found
    }
}

/// Handles the `get_class_hierarchy` tool.
pub fn handle_get_class_hierarchy(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;

    let graph = build_graph_from_path(path, registry)?;

    let mermaid = MermaidRenderer::new().render(&graph, DiagramKind::ClassDiagram);

    Ok(json!({
        "mermaid": mermaid
    }))
}

/// Handles the `find_entry_points` tool.
pub fn handle_find_entry_points(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;

    let graph = build_graph_from_path(path, registry)?;

    let mut has_incoming_calls = HashSet::new();
    for edge in &graph.edges {
        if edge.kind == EdgeKind::Calls {
            has_incoming_calls.insert(&edge.to_id);
        }
    }

    let entry_points: Vec<&Node> = graph
        .nodes
        .iter()
        .filter(|n| {
            matches!(n.kind, NodeKind::Function { .. }) && !has_incoming_calls.contains(&n.id)
        })
        .collect();

    Ok(json!({
        "nodes": entry_points
    }))
}

/// Explains the path between two nodes.
pub fn handle_explain_path(
    params: Value,
    registry: &LanguageRegistry,
) -> Result<Value, JsonRpcError> {
    let path_str = params
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'path' parameter"))?;
    let from_name = params
        .get("from")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'from' parameter"))?;
    let to_name = params
        .get("to")
        .and_then(|v| v.as_str())
        .ok_or_else(|| JsonRpcError::invalid_params("Missing 'to' parameter"))?;

    let graph = build_graph_from_path(path_str, registry)?;

    let from_node = graph
        .nodes
        .iter()
        .find(|n| n.label == from_name || n.id.ends_with(from_name));
    let to_node = graph
        .nodes
        .iter()
        .find(|n| n.label == to_name || n.id.ends_with(to_name));

    if from_node.is_none() || to_node.is_none() {
        return Ok(json!({
            "nodes": [],
            "exists": false
        }));
    }
    let from_node = from_node.expect("Node exists");
    let to_node = to_node.expect("Node exists");

    let mut adj = HashMap::new();
    for edge in &graph.edges {
        adj.entry(&edge.from_id)
            .or_insert_with(Vec::new)
            .push(&edge.to_id);
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();

    queue.push_back(&from_node.id);
    visited.insert(&from_node.id);

    let mut found = false;
    while let Some(curr) = queue.pop_front() {
        if curr == &to_node.id {
            found = true;
            break;
        }

        if let Some(neighbors) = adj.get(curr) {
            for &next in neighbors {
                if visited.insert(next) {
                    parent.insert(next, curr);
                    queue.push_back(next);
                }
            }
        }
    }

    if found {
        let mut path_ids = Vec::new();
        let mut curr = &to_node.id;
        path_ids.push(curr);
        while let Some(&p) = parent.get(curr) {
            path_ids.push(p);
            curr = p;
        }
        path_ids.reverse();

        let mut path_nodes = Vec::new();
        for id in path_ids {
            if let Some(n) = graph.nodes.iter().find(|n| &n.id == id) {
                path_nodes.push(n);
            }
        }

        Ok(json!({
            "nodes": path_nodes,
            "exists": true
        }))
    } else {
        Ok(json!({
            "nodes": [],
            "exists": false
        }))
    }
}

fn build_graph_from_path(
    dir_path: &str,
    registry: &LanguageRegistry,
) -> Result<CodeGraph, JsonRpcError> {
    let p = Path::new(dir_path);
    if !p.exists() {
        return Err(JsonRpcError::internal_error(&format!(
            "Path does not exist: {}",
            dir_path
        )));
    }

    let files = walk_dir(p).map_err(|e| JsonRpcError::internal_error(&e))?;

    let mut merged_graph = CodeGraph::new(GraphMeta {
        language: "mixed".to_string(),
        source_root: dir_path.to_string(),
        generated_at: "2024-01-01T00:00:00Z".to_string(),
        node_count: 0,
        edge_count: 0,
    });

    for file in files {
        if let Some(ext) = file.extension().and_then(|e| e.to_str())
            && registry.find_parser(ext).is_some()
            && let Ok(source) = std::fs::read_to_string(&file)
            && let Ok(graph) = registry.parse_file(&file.to_string_lossy(), &source)
        {
            merged_graph.nodes.extend(graph.nodes);
            merged_graph.edges.extend(graph.edges);
        }
    }

    merged_graph.meta.node_count = merged_graph.nodes.len();
    merged_graph.meta.edge_count = merged_graph.edges.len();

    Ok(merged_graph)
}

fn walk_dir(dir: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read dir {}: {}", dir.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                if !path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .starts_with('.')
                {
                    files.extend(walk_dir(&path)?);
                }
            } else {
                files.push(path);
            }
        }
    } else {
        files.push(dir.to_path_buf());
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codeviz_core::{CodeGraph, Edge, EdgeKind, GraphMeta, LanguageRegistry, Node, NodeKind};
    use serde_json::json;

    // Helper to create a mock graph directly since build_graph_from_path reads from disk
    // In actual tools, they call build_graph_from_path.
    // To test the logic WITHOUT hitting disk, we should refactor the handlers to take a `&CodeGraph` directly,
    // or test using a temp directory. A temp directory is easiest for not changing the signature.

    use std::fs;
    use tempfile::tempdir;

    struct DummyParser;
    impl codeviz_core::parser::LanguageParser for DummyParser {
        fn language_name(&self) -> &str {
            "dummy"
        }
        fn supported_extensions(&self) -> &[&str] {
            &["dummy"]
        }
        fn parse(
            &self,
            _source: &str,
            file_path: &str,
        ) -> Result<CodeGraph, codeviz_core::parser::ParseError> {
            let mut graph = CodeGraph::new(GraphMeta {
                language: "dummy".to_string(),
                source_root: "/".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            });

            graph.nodes.push(Node {
                id: format!("{}::A", file_path),
                label: "A".to_string(),
                kind: NodeKind::Function { is_async: false },
                file_path: file_path.to_string(),
                line: None,
                is_public: true,
            });
            graph.nodes.push(Node {
                id: format!("{}::B", file_path),
                label: "B".to_string(),
                kind: NodeKind::Function { is_async: false },
                file_path: file_path.to_string(),
                line: None,
                is_public: true,
            });
            graph.edges.push(Edge {
                from_id: format!("{}::A", file_path),
                to_id: format!("{}::B", file_path),
                kind: EdgeKind::Calls,
            });

            graph.meta.node_count = graph.nodes.len();
            graph.meta.edge_count = graph.edges.len();

            Ok(graph)
        }
    }

    fn setup_registry() -> LanguageRegistry {
        let mut reg = LanguageRegistry::new();
        reg.register(Box::new(DummyParser));
        reg
    }

    #[test]
    fn test_list_tools() {
        let tools = list_tools();
        assert_eq!(tools.len(), 6);
        assert!(tools.iter().any(|t| t.name == "get_module_graph"));
    }

    #[test]
    fn test_handle_get_callers() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.dummy");
        fs::write(&file_path, "dummy code").unwrap();

        let reg = setup_registry();

        let params = json!({
            "path": dir.path().to_str().unwrap(),
            "fn_name": "B"
        });

        let res = handle_get_callers(params, &reg).unwrap();
        let callers = res.get("callers").unwrap().as_array().unwrap();
        assert_eq!(callers.len(), 1);
        assert_eq!(callers[0].get("label").unwrap().as_str().unwrap(), "A");
    }

    #[test]
    fn test_handle_get_callees() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.dummy");
        fs::write(&file_path, "dummy code").unwrap();

        let reg = setup_registry();

        let params = json!({
            "path": dir.path().to_str().unwrap(),
            "fn_name": "A"
        });

        let res = handle_get_callees(params, &reg).unwrap();
        let callees = res.get("callees").unwrap().as_array().unwrap();
        assert_eq!(callees.len(), 1);
        assert_eq!(callees[0].get("label").unwrap().as_str().unwrap(), "B");
    }

    #[test]
    fn test_handle_find_entry_points() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.dummy");
        fs::write(&file_path, "dummy code").unwrap();

        let reg = setup_registry();

        let params = json!({
            "path": dir.path().to_str().unwrap()
        });

        let res = handle_find_entry_points(params, &reg).unwrap();
        let nodes = res.get("nodes").unwrap().as_array().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].get("label").unwrap().as_str().unwrap(), "A");
    }
}
