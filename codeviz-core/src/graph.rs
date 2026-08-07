use serde::{Deserialize, Serialize};

/// Represents a language-agnostic Intermediate Representation (IR) graph.
/// This graph serves as the central data contract for CodeViz.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGraph {
    /// The nodes present in the graph.
    pub nodes: Vec<Node>,
    /// The edges connecting the nodes in the graph.
    pub edges: Vec<Edge>,
    /// Metadata associated with the graph.
    pub meta: GraphMeta,
}

/// Represents a node within the code graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    /// Globally unique identifier: "{file_path}::{symbol_name}".
    pub id: String,
    /// Display name, stripped of generics/lifetimes.
    pub label: String,
    /// The kind of the node.
    pub kind: NodeKind,
    /// File path relative to source_root.
    pub file_path: String,
    /// 1-indexed line number; None if not resolvable.
    pub line: Option<u32>,
    /// True if exported/pub.
    pub is_public: bool,
}

/// Defines the kind of a code node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    /// A file node.
    File,
    /// A module node.
    Module,
    /// A function node, indicating if it's asynchronous.
    Function {
        /// Whether the function is async.
        is_async: bool,
    },
    /// A class node.
    Class,
    /// An interface node.
    Interface,
    /// A constant node.
    Constant,
}

/// Represents a directional edge between two nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// The ID of the node where this edge originates. Must match a Node.id.
    pub from_id: String,
    /// The ID of the node where this edge terminates. Must match a Node.id, or be an unresolved external.
    pub to_id: String,
    /// The kind of the edge.
    pub kind: EdgeKind,
}

/// Defines the kind of relationship represented by an edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// Module-level dependency.
    Imports,
    /// Function invokes function.
    Calls,
    /// Class extends class / trait extends trait.
    Inherits,
    /// Class implements interface / impl Trait for Struct.
    Implements,
    /// Function returns type.
    Returns,
    /// Function creates instance of class.
    Instantiates,
}

/// Metadata describing the code graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphMeta {
    /// The language of the parsed codebase (e.g., "python", "typescript").
    pub language: String,
    /// Absolute path that was scanned.
    pub source_root: String,
    /// ISO 8601 UTC timestamp of generation.
    pub generated_at: String,
    /// The total number of nodes. Must equal `nodes.len()`.
    pub node_count: usize,
    /// The total number of edges. Must equal `edges.len()`.
    pub edge_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_empty_graph() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![],
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "/foo".to_string(),
                generated_at: "2024-01-01T00:00:00Z".to_string(),
                node_count: 0,
                edge_count: 0,
            },
        };

        let serialized = serde_json::to_string(&graph).expect("failed to serialize");
        let deserialized: CodeGraph = serde_json::from_str(&serialized).expect("failed to deserialize");

        assert_eq!(graph, deserialized);
    }

    #[test]
    fn test_roundtrip_full_graph() {
        let graph = CodeGraph {
            nodes: vec![
                Node {
                    id: "src/main.rs::main".to_string(),
                    label: "main".to_string(),
                    kind: NodeKind::Function { is_async: true },
                    file_path: "src/main.rs".to_string(),
                    line: Some(10),
                    is_public: true,
                },
                Node {
                    id: "src/foo.rs::Foo".to_string(),
                    label: "Foo".to_string(),
                    kind: NodeKind::Class,
                    file_path: "src/foo.rs".to_string(),
                    line: Some(5),
                    is_public: false,
                },
            ],
            edges: vec![Edge {
                from_id: "src/main.rs::main".to_string(),
                to_id: "src/foo.rs::Foo".to_string(),
                kind: EdgeKind::Instantiates,
            }],
            meta: GraphMeta {
                language: "rust".to_string(),
                source_root: "/project".to_string(),
                generated_at: "2024-01-01T00:00:00Z".to_string(),
                node_count: 2,
                edge_count: 1,
            },
        };

        let serialized = serde_json::to_string(&graph).expect("failed to serialize");
        let deserialized: CodeGraph = serde_json::from_str(&serialized).expect("failed to deserialize");

        assert_eq!(graph, deserialized);

        // Explicitly check the Function async serialization structure if possible
        assert!(serialized.contains(r#"{"Function":{"is_async":true}}"#));
    }
}
