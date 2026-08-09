use codeviz_core::{
    CodeGraph,
    render::mermaid::{DiagramKind, MermaidRenderer},
};
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
}
