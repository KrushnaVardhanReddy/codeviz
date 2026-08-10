use crate::{CodeGraph, EdgeKind, NodeKind};

/// Defines the kind of Mermaid diagram to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagramKind {
    /// Renders a 'graph TD' showing only 'Imports' edges between 'File'/'Module' nodes.
    ModuleGraph,
    /// Renders a 'flowchart TD' showing only 'Calls' edges between 'Function' nodes.
    CallGraph,
    /// Renders a 'classDiagram' showing 'Inherits' and 'Implements' edges.
    ClassDiagram,
}

/// A renderer for converting a `CodeGraph` into a Mermaid diagram string.
pub struct MermaidRenderer;

impl Default for MermaidRenderer {
    fn default() -> Self {
        Self
    }
}

impl MermaidRenderer {
    /// Creates a new `MermaidRenderer`.
    pub fn new() -> Self {
        Self
    }

    /// Renders the given `CodeGraph` into a Mermaid diagram string of the specified `DiagramKind`.
    pub fn render(&self, graph: &CodeGraph, kind: DiagramKind) -> String {
        let mut output = String::new();

        // Edge case: if empty graph
        if graph.nodes.is_empty() {
            let header = match kind {
                DiagramKind::ModuleGraph => "graph TD\n",
                DiagramKind::CallGraph => "flowchart TD\n",
                DiagramKind::ClassDiagram => "classDiagram\n",
            };
            output.push_str(header);
            return output;
        }

        let mut filtered_nodes: Vec<&crate::Node> = graph
            .nodes
            .iter()
            .filter(|n| match kind {
                DiagramKind::ModuleGraph => matches!(n.kind, NodeKind::File | NodeKind::Module),
                DiagramKind::CallGraph => matches!(n.kind, NodeKind::Function { .. }),
                DiagramKind::ClassDiagram => {
                    matches!(n.kind, NodeKind::Class | NodeKind::Interface)
                }
            })
            .collect();

        let filtered_count = filtered_nodes.len();

        if filtered_count > 50 {
            output.push_str("%% WARNING: graph truncated at 50 nodes\n");
            filtered_nodes.truncate(50);
        }

        let header = match kind {
            DiagramKind::ModuleGraph => "graph TD\n",
            DiagramKind::CallGraph => "flowchart TD\n",
            DiagramKind::ClassDiagram => "classDiagram\n",
        };
        output.push_str(header);

        let valid_from_ids: std::collections::HashSet<&str> =
            filtered_nodes.iter().map(|n| n.id.as_str()).collect();

        for edge in &graph.edges {
            if !valid_from_ids.contains(edge.from_id.as_str()) {
                continue; // from must be in filtered nodes
            }

            let include_edge = match kind {
                DiagramKind::ModuleGraph => matches!(edge.kind, EdgeKind::Imports),
                DiagramKind::CallGraph => matches!(edge.kind, EdgeKind::Calls),
                DiagramKind::ClassDiagram => {
                    matches!(edge.kind, EdgeKind::Inherits | EdgeKind::Implements)
                }
            };

            if include_edge {
                let from = Self::sanitize_id(&edge.from_id);
                let to = Self::sanitize_id(&edge.to_id);

                match kind {
                    DiagramKind::ModuleGraph | DiagramKind::CallGraph => {
                        output.push_str(&format!("    {} --> {}\n", from, to));
                    }
                    DiagramKind::ClassDiagram => {
                        // Class diagram: Animal <|-- Dog => to <|-- from
                        match edge.kind {
                            EdgeKind::Inherits => {
                                output.push_str(&format!("    {} <|-- {}\n", to, from))
                            }
                            EdgeKind::Implements => {
                                output.push_str(&format!("    {} <|.. {}\n", to, from))
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        output
    }

    pub(crate) fn sanitize_id(id: &str) -> String {
        id.replace("/", "_").replace(".", "_").replace("::", "_")
    }

    /// Renders a diff diagram highlighting added and removed nodes and edges.
    pub fn render_diff(&self, diff: &crate::diff::GraphDiff, kind: DiagramKind) -> String {
        let mut output = String::new();

        let header = match kind {
            DiagramKind::ModuleGraph => "graph TD\n",
            DiagramKind::CallGraph => "flowchart TD\n",
            DiagramKind::ClassDiagram => "classDiagram\n",
        };
        output.push_str(header);

        output.push_str("    classDef added fill:#aaffaa,stroke:#00aa00;\n");
        output.push_str("    classDef removed fill:#ffaaaa,stroke:#aa0000;\n");

        for node in &diff.added_nodes {
            let sanitized_id = Self::sanitize_id(&node.id);
            output.push_str(&format!("    class {} added;\n", sanitized_id));
        }

        for node in &diff.removed_nodes {
            let sanitized_id = Self::sanitize_id(&node.id);
            output.push_str(&format!("    class {} removed;\n", sanitized_id));
        }

        let valid_edge = |edge: &crate::Edge| -> bool {
            match kind {
                DiagramKind::ModuleGraph => matches!(edge.kind, EdgeKind::Imports),
                DiagramKind::CallGraph => matches!(edge.kind, EdgeKind::Calls),
                DiagramKind::ClassDiagram => {
                    matches!(edge.kind, EdgeKind::Inherits | EdgeKind::Implements)
                }
            }
        };

        for edge in &diff.added_edges {
            if valid_edge(edge) {
                let from = Self::sanitize_id(&edge.from_id);
                let to = Self::sanitize_id(&edge.to_id);
                match kind {
                    DiagramKind::ModuleGraph | DiagramKind::CallGraph => {
                        output.push_str(&format!("    {} -->|added| {}:::added\n", from, to));
                    }
                    DiagramKind::ClassDiagram => match edge.kind {
                        EdgeKind::Inherits => {
                            output.push_str(&format!("    {} <|-- {}:::added\n", to, from))
                        }
                        EdgeKind::Implements => {
                            output.push_str(&format!("    {} <|.. {}:::added\n", to, from))
                        }
                        _ => {}
                    },
                }
            }
        }

        for edge in &diff.removed_edges {
            if valid_edge(edge) {
                let from = Self::sanitize_id(&edge.from_id);
                let to = Self::sanitize_id(&edge.to_id);
                match kind {
                    DiagramKind::ModuleGraph | DiagramKind::CallGraph => {
                        output.push_str(&format!("    {} -->|removed| {}:::removed\n", from, to));
                    }
                    DiagramKind::ClassDiagram => match edge.kind {
                        EdgeKind::Inherits => {
                            output.push_str(&format!("    {} <|-- {}:::removed\n", to, from))
                        }
                        EdgeKind::Implements => {
                            output.push_str(&format!("    {} <|.. {}:::removed\n", to, from))
                        }
                        _ => {}
                    },
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, GraphMeta, Node};

    fn make_graph() -> CodeGraph {
        CodeGraph {
            nodes: vec![
                Node {
                    id: "src/main.rs::main".to_string(),
                    label: "main".to_string(),
                    kind: NodeKind::Function { is_async: false },
                    file_path: "src/main.rs".to_string(),
                    line: Some(1),
                    is_public: true,
                },
                Node {
                    id: "src/parser.rs::parse".to_string(),
                    label: "parse".to_string(),
                    kind: NodeKind::Function { is_async: false },
                    file_path: "src/parser.rs".to_string(),
                    line: Some(10),
                    is_public: true,
                },
                Node {
                    id: "src/config.rs::Config".to_string(),
                    label: "Config".to_string(),
                    kind: NodeKind::Class,
                    file_path: "src/config.rs".to_string(),
                    line: Some(5),
                    is_public: true,
                },
            ],
            edges: vec![
                Edge {
                    from_id: "src/main.rs::main".to_string(),
                    to_id: "src/parser.rs::parse".to_string(),
                    kind: EdgeKind::Calls,
                },
                Edge {
                    from_id: "src/main.rs::main".to_string(),
                    to_id: "src/config.rs::Config".to_string(),
                    kind: EdgeKind::Instantiates,
                },
            ],
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "/app".to_string(),
                generated_at: "2023-10-10T00:00:00Z".to_string(),
                node_count: 3,
                edge_count: 2,
            },
            control_flow: None,
        }
    }

    #[test]
    fn test_sanitization() {
        assert_eq!(
            MermaidRenderer::sanitize_id("src/main.rs::foo"),
            "src_main_rs_foo"
        );
        assert_eq!(MermaidRenderer::sanitize_id("a.b/c::d"), "a_b_c_d");
    }

    #[test]
    fn test_empty_graph() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![],
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            },
            control_flow: None,
        };
        let renderer = MermaidRenderer::new();
        assert_eq!(
            renderer.render(&graph, DiagramKind::ModuleGraph),
            "graph TD\n"
        );
    }

    #[test]
    fn test_call_graph() {
        let graph = make_graph();
        let renderer = MermaidRenderer::new();
        let out = renderer.render(&graph, DiagramKind::CallGraph);
        assert!(out.starts_with("flowchart TD\n"));
        // Should only contain Function nodes and Call edges
        assert!(out.contains("src_main_rs_main --> src_parser_rs_parse"));
        // Should not contain Instantiates edge
        assert!(!out.contains("src_config_rs_Config"));
    }

    #[test]
    fn test_truncation() {
        let mut nodes = vec![];
        for i in 0..55 {
            nodes.push(Node {
                id: format!("node_{}", i),
                label: format!("Node {}", i),
                kind: NodeKind::File,
                file_path: "path".to_string(),
                line: None,
                is_public: true,
            });
        }
        let graph = CodeGraph {
            nodes,
            edges: vec![],
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 55,
                edge_count: 0,
            },
            control_flow: None,
        };

        let renderer = MermaidRenderer::new();
        let out = renderer.render(&graph, DiagramKind::ModuleGraph);
        assert!(out.starts_with("%% WARNING: graph truncated at 50 nodes\n"));
        assert!(out.contains("graph TD\n"));
    }

    #[test]
    fn test_module_graph() {
        let graph = CodeGraph {
            nodes: vec![
                Node {
                    id: "src/main.rs".to_string(),
                    label: "main".to_string(),
                    kind: NodeKind::File,
                    file_path: "src/main.rs".to_string(),
                    line: None,
                    is_public: true,
                },
                Node {
                    id: "src/config.rs".to_string(),
                    label: "config".to_string(),
                    kind: NodeKind::File,
                    file_path: "src/config.rs".to_string(),
                    line: None,
                    is_public: true,
                },
            ],
            edges: vec![Edge {
                from_id: "src/main.rs".to_string(),
                to_id: "src/config.rs".to_string(),
                kind: EdgeKind::Imports,
            }],
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "/app".to_string(),
                generated_at: "2023-10-10T00:00:00Z".to_string(),
                node_count: 2,
                edge_count: 1,
            },
            control_flow: None,
        };
        let renderer = MermaidRenderer::new();
        let out = renderer.render(&graph, DiagramKind::ModuleGraph);
        assert!(out.starts_with("graph TD\n"));
        assert!(out.contains("src_main_rs --> src_config_rs"));
    }

    #[test]
    fn test_class_diagram() {
        let graph = CodeGraph {
            nodes: vec![
                Node {
                    id: "Dog".to_string(),
                    label: "Dog".to_string(),
                    kind: NodeKind::Class,
                    file_path: "src/main.rs".to_string(),
                    line: None,
                    is_public: true,
                },
                Node {
                    id: "Animal".to_string(),
                    label: "Animal".to_string(),
                    kind: NodeKind::Class,
                    file_path: "src/main.rs".to_string(),
                    line: None,
                    is_public: true,
                },
            ],
            edges: vec![Edge {
                from_id: "Dog".to_string(),
                to_id: "Animal".to_string(),
                kind: EdgeKind::Inherits,
            }],
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "/app".to_string(),
                generated_at: "2023-10-10T00:00:00Z".to_string(),
                node_count: 2,
                edge_count: 1,
            },
            control_flow: None,
        };
        let renderer = MermaidRenderer::new();
        let out = renderer.render(&graph, DiagramKind::ClassDiagram);
        assert!(out.starts_with("classDiagram\n"));
        assert!(out.contains("Animal <|-- Dog\n"));
    }

    #[test]
    fn test_render_diff() {
        let mut diff = crate::diff::GraphDiff {
            added_nodes: vec![],
            removed_nodes: vec![],
            added_edges: vec![],
            removed_edges: vec![],
        };

        let node_added = crate::Node {
            id: "src/new.rs".to_string(),
            label: "new.rs".to_string(),
            kind: crate::NodeKind::File,
            file_path: "src/new.rs".to_string(),
            line: None,
            is_public: true,
        };
        diff.added_nodes.push(node_added);

        let renderer = MermaidRenderer::new();
        let result = renderer.render_diff(&diff, DiagramKind::ModuleGraph);
        assert!(result.contains("graph TD"));
        assert!(result.contains("class src_new_rs added;"));
        assert!(result.contains("classDef added fill:#aaffaa"));
    }
}
