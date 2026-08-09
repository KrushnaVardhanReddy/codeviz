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
pub fn parse_and_build_graph(language: &str, file_path: &str, ast_json: &str) -> Result<String, String> {
    let root_node: TsNode = serde_json::from_str(ast_json).map_err(|e| format!("Failed to parse AST JSON: {}", e))?;

    let mut graph = CodeGraph::new(GraphMeta {
        language: language.to_string(),
        source_root: "".to_string(),
        generated_at: "".to_string(),
        node_count: 0,
        edge_count: 0,
    });

    graph.nodes.push(Node {
        id: file_path.to_string(),
        label: file_path.split('/').next_back().unwrap_or(file_path).to_string(),
        kind: NodeKind::File,
        file_path: file_path.to_string(),
        line: None,
        is_public: true,
    });

    // Very basic extraction of functions and classes based on tree structure.
    extract_from_ast(&root_node, file_path, &mut graph);

    graph.meta.node_count = graph.nodes.len();
    graph.meta.edge_count = graph.edges.len();

    serde_json::to_string(&graph).map_err(|e| e.to_string())
}

fn extract_from_ast(node: &TsNode, file_path: &str, graph: &mut CodeGraph) {
    let is_function = node.node_type == "function_definition" || node.node_type == "function_declaration" || node.node_type == "method_definition" || node.node_type == "arrow_function";
    let is_class = node.node_type == "class_definition" || node.node_type == "class_declaration";

    if is_function || is_class {
        // Try to find identifier child
        let identifier = node.children.iter().find(|c| c.node_type == "identifier" || c.node_type == "property_identifier" || c.node_type == "type_identifier");
        if let Some(id_node) = identifier {
            let name = &id_node.text;
            let id = format!("{}::{}", file_path, name);

            graph.nodes.push(Node {
                id,
                label: name.clone(),
                kind: if is_function { NodeKind::Function { is_async: false } } else { NodeKind::Class },
                file_path: file_path.to_string(),
                line: Some(node.start_position.row + 1),
                is_public: true,
            });
        }
    }

    for child in &node.children {
        extract_from_ast(child, file_path, graph);
    }
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
}
