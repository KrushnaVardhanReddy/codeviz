use codeviz_core::{
    CodeGraph,
    ir::{GraphMeta, Node, NodeKind},
    render::mermaid::{DiagramKind, MermaidRenderer},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

fn get_diagram_kind(kind: &str) -> Result<DiagramKind, String> {
    match kind {
        "module" => Ok(DiagramKind::ModuleGraph),
        "call" => Ok(DiagramKind::CallGraph),
        "class" => Ok(DiagramKind::ClassDiagram),
        _ => Err(format!("Unknown diagram kind: {}", kind)),
    }
}

/// Render a pre-parsed CodeGraph JSON into a Mermaid diagram string.
/// @param graph_json   - Full JSON-serialized CodeGraph
/// @param diagram_kind - "module" | "call" | "class"
/// @returns Mermaid diagram string
/// @throws string error message on failure
#[wasm_bindgen]
pub fn render_graph(graph_json: &str, diagram_kind: &str) -> Result<String, String> {
    let kind = get_diagram_kind(diagram_kind)?;
    let graph: CodeGraph = serde_json::from_str(graph_json).map_err(|e| e.to_string())?;

    let renderer = MermaidRenderer::new();
    Ok(renderer.render(&graph, kind))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub text: String,
    pub start_position: Position,
    pub end_position: Position,
    pub children: Vec<TsNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub row: u32,
    pub column: u32,
}

/// A simplified AST to CodeGraph generator using JSON AST created by web-tree-sitter.
#[wasm_bindgen]
pub fn parse_and_build_graph(
    language: &str,
    file_path: &str,
    ast_json: &str,
) -> Result<String, String> {
    let root_node: TsNode =
        serde_json::from_str(ast_json).map_err(|e| format!("Failed to parse AST JSON: {}", e))?;

    let mut graph = CodeGraph::new(GraphMeta {
        language: language.to_string(),
        source_root: "".to_string(),
        generated_at: "".to_string(),
        node_count: 0,
        edge_count: 0,
    });

    graph.nodes.push(Node {
        id: file_path.to_string(),
        label: file_path
            .split('/')
            .next_back()
            .unwrap_or(file_path)
            .to_string(),
        kind: NodeKind::File,
        file_path: file_path.to_string(),
        line: None,
        is_public: true,
        parent_id: None,
    });

    // Very basic extraction of functions and classes based on tree structure.
    extract_from_ast(&root_node, file_path, &mut graph, Some(file_path));

    graph.meta.node_count = graph.nodes.len();
    graph.meta.edge_count = graph.edges.len();

    serde_json::to_string(&graph).map_err(|e| e.to_string())
}

fn extract_from_ast(node: &TsNode, file_path: &str, graph: &mut CodeGraph, current_scope_id: Option<&str>) {
    let mut is_function = node.node_type == "function_definition"
        || node.node_type == "function_declaration"
        || node.node_type == "method_definition"
        || node.node_type == "arrow_function";
    let is_class = node.node_type == "class_definition" || node.node_type == "class_declaration";
    let is_call = node.node_type == "call_expression" || node.node_type == "call"
        || node.node_type == "jsx_self_closing_element" || node.node_type == "jsx_opening_element";

    if node.node_type == "variable_declarator"
        && node.children.iter().any(|c| c.node_type == "arrow_function") {
            is_function = true;
        }

    let mut next_scope_id = current_scope_id.map(|s| s.to_string());

    if is_function || is_class {
        // Try to find identifier child
        let identifier = node.children.iter().find(|c| {
            c.node_type == "identifier"
                || c.node_type == "property_identifier"
                || c.node_type == "type_identifier"
        });
        if let Some(id_node) = identifier {
            let name = &id_node.text;
            let id = format!("{}::{}", file_path, name);
            next_scope_id = Some(id.clone());

            graph.nodes.push(Node {
                id: id.clone(),
                label: name.clone(),
                kind: if is_function {
                    NodeKind::Function { is_async: false }
                } else {
                    NodeKind::Class
                },
                file_path: file_path.to_string(),
                line: Some(node.start_position.row + 1),
                is_public: true,
                parent_id: current_scope_id.map(|s| s.to_string()),
            });

            // Emit a Contains edge from the parent scope (file or class) to this node
            if let Some(parent_scope) = current_scope_id {
                graph.edges.push(codeviz_core::ir::Edge {
                    from_id: parent_scope.to_string(),
                    to_id: id.clone(),
                    kind: codeviz_core::ir::EdgeKind::Contains,
                });
            }

            if is_class {
                for child in &node.children {
                    if child.node_type == "argument_list" || child.node_type == "class_heritage" {
                        let mut base_names = Vec::new();
                        find_identifiers(child, &mut base_names);
                        for base_name in base_names {
                            graph.edges.push(codeviz_core::ir::Edge {
                                from_id: id.clone(),
                                to_id: format!("{}::{}", file_path, base_name),
                                kind: codeviz_core::ir::EdgeKind::Inherits,
                            });
                        }
                    }
                }
            }
        }
    } else if is_call
        && let Some(scope_id) = current_scope_id {
            let mut target_name = None;
            let is_jsx = node.node_type.starts_with("jsx");

            if is_jsx {
                let mut identifiers = Vec::new();
                find_identifiers(node, &mut identifiers);
                if let Some(first_id) = identifiers.first() {
                    target_name = Some(first_id.clone());
                }
            } else {
                for child in &node.children {
                    if child.node_type == "attribute" || child.node_type == "member_expression" {
                        let mut identifiers = Vec::new();
                        find_identifiers(child, &mut identifiers);
                        if let Some(last_id) = identifiers.last() {
                            target_name = Some(last_id.clone());
                        }
                    } else if (child.node_type == "identifier" || child.node_type == "property_identifier")
                        && target_name.is_none() {
                            target_name = Some(child.text.clone());
                        }
                }
            }

            if let Some(name) = target_name {
                let target_id = format!("{}::{}", file_path, name);
                let edge_kind = if is_jsx { codeviz_core::ir::EdgeKind::Instantiates } else { codeviz_core::ir::EdgeKind::Calls };
                
                graph.edges.push(codeviz_core::ir::Edge {
                    from_id: scope_id.to_string(),
                    to_id: target_id,
                    kind: edge_kind,
                });
            }
        }

    for child in &node.children {
        extract_from_ast(child, file_path, graph, next_scope_id.as_deref());
    }
}

fn find_identifiers(node: &TsNode, identifiers: &mut Vec<String>) {
    if node.node_type == "identifier"
        || node.node_type == "property_identifier"
        || node.node_type == "type_identifier"
    {
        identifiers.push(node.text.clone());
    }
    for child in &node.children {
        find_identifiers(child, identifiers);
    }
}

/// Return a list of supported language identifiers.
#[wasm_bindgen]
pub fn supported_languages() -> js_sys::Array {
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str("python"));
    arr.push(&JsValue::from_str("typescript"));
    arr.push(&JsValue::from_str("javascript"));
    arr.push(&JsValue::from_str("go"));
    arr.push(&JsValue::from_str("rust"));
    arr.push(&JsValue::from_str("java"));
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_graph_module() {
        let json_graph = r#"{
            "nodes": [
                {
                    "id": "file.py",
                    "label": "file.py",
                    "kind": "File",
                    "file_path": "file.py",
                    "line": null,
                    "is_public": true
                }
            ],
            "edges": [],
            "meta": {
                "language": "python",
                "source_root": "",
                "generated_at": "",
                "node_count": 1,
                "edge_count": 0
            }
        }"#;

        let result = render_graph(json_graph, "module");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.starts_with("graph TD\n"));
    }

    #[test]
    fn test_invalid_json() {
        let result = render_graph("{ invalid json }", "module");
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_diagram_kind() {
        let json_graph = r#"{ "nodes": [], "edges": [], "meta": { "language": "python", "source_root": "", "generated_at": "", "node_count": 0, "edge_count": 0 } }"#;
        let result = render_graph(json_graph, "unknown");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown diagram kind: unknown");
    }

    #[test]
    fn test_parse_and_build_graph() {
        let ast = r#"{
            "type": "program",
            "text": "",
            "start_position": {"row": 0, "column": 0},
            "end_position": {"row": 10, "column": 0},
            "children": [
                {
                    "type": "function_definition",
                    "text": "def test(): pass",
                    "start_position": {"row": 0, "column": 0},
                    "end_position": {"row": 1, "column": 0},
                    "children": [
                        {
                            "type": "identifier",
                            "text": "test",
                            "start_position": {"row": 0, "column": 4},
                            "end_position": {"row": 0, "column": 8},
                            "children": []
                        }
                    ]
                }
            ]
        }"#;
        let res = parse_and_build_graph("python", "test.py", ast);
        assert!(res.is_ok());
        let json = res.unwrap();
        assert!(json.contains("test.py::test"));
    }

    #[test]
    fn test_parse_and_build_graph_inheritance() {
        let ast = r#"{
            "type": "program",
            "text": "",
            "start_position": {"row": 0, "column": 0},
            "end_position": {"row": 10, "column": 0},
            "children": [
                {
                    "type": "class_definition",
                    "text": "class Dog(Animal): pass",
                    "start_position": {"row": 0, "column": 0},
                    "end_position": {"row": 1, "column": 0},
                    "children": [
                        {
                            "type": "identifier",
                            "text": "Dog",
                            "start_position": {"row": 0, "column": 6},
                            "end_position": {"row": 0, "column": 9},
                            "children": []
                        },
                        {
                            "type": "argument_list",
                            "text": "(Animal)",
                            "start_position": {"row": 0, "column": 9},
                            "end_position": {"row": 0, "column": 17},
                            "children": [
                                {
                                    "type": "identifier",
                                    "text": "Animal",
                                    "start_position": {"row": 0, "column": 10},
                                    "end_position": {"row": 0, "column": 16},
                                    "children": []
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;
        let res = parse_and_build_graph("python", "test.py", ast);
        assert!(res.is_ok());
        let json = res.unwrap();
        let graph: CodeGraph = serde_json::from_str(&json).unwrap();

        let inherits_edge = graph.edges.iter().find(|e| e.kind == codeviz_core::ir::EdgeKind::Inherits);
        assert!(inherits_edge.is_some());
        let edge = inherits_edge.unwrap();
        assert_eq!(edge.from_id, "test.py::Dog");
        assert_eq!(edge.to_id, "test.py::Animal");
    }

    #[test]
    fn test_parse_and_build_graph_method_call() {
        let ast = r#"{
            "type": "program",
            "text": "",
            "start_position": {"row": 0, "column": 0},
            "end_position": {"row": 10, "column": 0},
            "children": [
                {
                    "type": "function_definition",
                    "text": "def test(): d.bark()",
                    "start_position": {"row": 0, "column": 0},
                    "end_position": {"row": 1, "column": 0},
                    "children": [
                        {
                            "type": "identifier",
                            "text": "test",
                            "start_position": {"row": 0, "column": 4},
                            "end_position": {"row": 0, "column": 8},
                            "children": []
                        },
                        {
                            "type": "call",
                            "text": "d.bark()",
                            "start_position": {"row": 0, "column": 12},
                            "end_position": {"row": 0, "column": 20},
                            "children": [
                                {
                                    "type": "attribute",
                                    "text": "d.bark",
                                    "start_position": {"row": 0, "column": 12},
                                    "end_position": {"row": 0, "column": 18},
                                    "children": [
                                        {
                                            "type": "identifier",
                                            "text": "d",
                                            "start_position": {"row": 0, "column": 12},
                                            "end_position": {"row": 0, "column": 13},
                                            "children": []
                                        },
                                        {
                                            "type": "identifier",
                                            "text": "bark",
                                            "start_position": {"row": 0, "column": 14},
                                            "end_position": {"row": 0, "column": 18},
                                            "children": []
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ]
        }"#;
        let res = parse_and_build_graph("python", "test.py", ast);
        assert!(res.is_ok());
        let json = res.unwrap();
        let graph: CodeGraph = serde_json::from_str(&json).unwrap();

        let calls_edge = graph.edges.iter().find(|e| e.kind == codeviz_core::ir::EdgeKind::Calls);
        assert!(calls_edge.is_some());
        let edge = calls_edge.unwrap();
        assert_eq!(edge.from_id, "test.py::test");
        assert_eq!(edge.to_id, "test.py::bark");
    }
}
