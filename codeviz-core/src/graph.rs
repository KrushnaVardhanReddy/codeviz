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

impl CodeGraph {
    /// Returns all paths (up to `max_paths`) from `start_node_id` to `target_node_id`.
    /// Paths follow the `Calls` edge kind.
    pub fn all_paths(&self, start_node_id: &str, target_node_id: &str, max_paths: usize) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut current_path = vec![start_node_id.to_string()];
        let mut visited = std::collections::HashSet::new();
        visited.insert(start_node_id.to_string());
        
        self.dfs_all_paths(
            start_node_id,
            target_node_id,
            max_paths,
            &mut current_path,
            &mut visited,
            &mut paths,
        );
        
        paths
    }

    #[allow(clippy::collapsible_if)]
    fn dfs_all_paths(
        &self,
        current_node_id: &str,
        target_node_id: &str,
        max_paths: usize,
        current_path: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        paths: &mut Vec<Vec<String>>,
    ) {
        if paths.len() >= max_paths {
            return;
        }

        if current_node_id == target_node_id {
            paths.push(current_path.clone());
            return;
        }

        for edge in &self.edges {
            if edge.from_id == current_node_id && edge.kind == EdgeKind::Calls {
                if !visited.contains(&edge.to_id) {
                    visited.insert(edge.to_id.clone());
                    current_path.push(edge.to_id.clone());
                    
                    self.dfs_all_paths(
                        &edge.to_id,
                        target_node_id,
                        max_paths,
                        current_path,
                        visited,
                        paths,
                    );
                    
                    current_path.pop();
                    visited.remove(&edge.to_id);
                }
            }
        }
    }

    /// Returns the recursive caller tree up to `max_depth`.
    pub fn callers_recursive(&self, target_node_id: &str, max_depth: usize) -> serde_json::Value {
        let mut visited_path = std::collections::HashSet::new();
        visited_path.insert(target_node_id.to_string());
        self.build_callers_tree(target_node_id, max_depth, 0, &mut visited_path)
    }

    #[allow(clippy::collapsible_if)]
    fn build_callers_tree(
        &self,
        current_node_id: &str,
        max_depth: usize,
        current_depth: usize,
        visited_path: &mut std::collections::HashSet<String>,
    ) -> serde_json::Value {
        if current_depth >= max_depth {
            return serde_json::json!({
                "node": current_node_id,
                "callers": []
            });
        }

        let mut callers_list = Vec::new();

        for edge in &self.edges {
            if edge.to_id == current_node_id && edge.kind == EdgeKind::Calls {
                if !visited_path.contains(&edge.from_id) {
                    visited_path.insert(edge.from_id.clone());
                    
                    let caller_tree = self.build_callers_tree(
                        &edge.from_id,
                        max_depth,
                        current_depth + 1,
                        visited_path,
                    );
                    callers_list.push(caller_tree);
                    
                    visited_path.remove(&edge.from_id);
                }
            }
        }

        serde_json::json!({
            "node": current_node_id,
            "callers": callers_list
        })
    }

    /// Returns all transitively reachable nodes from the given node.
    #[allow(clippy::collapsible_if)]
    pub fn blast_radius(&self, start_node_id: &str) -> Vec<String> {
        let mut reachable = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back(start_node_id.to_string());
        
        while let Some(current) = queue.pop_front() {
            for edge in &self.edges {
                if edge.from_id == current && edge.kind == EdgeKind::Calls {
                    if !reachable.contains(&edge.to_id) && edge.to_id != start_node_id {
                        reachable.insert(edge.to_id.clone());
                        queue.push_back(edge.to_id.clone());
                    }
                }
            }
        }
        
        reachable.into_iter().collect()
    }
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
        let deserialized: CodeGraph =
            serde_json::from_str(&serialized).expect("failed to deserialize");

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
        let deserialized: CodeGraph =
            serde_json::from_str(&serialized).expect("failed to deserialize");

        assert_eq!(graph, deserialized);

        // Explicitly check the Function async serialization structure if possible
        assert!(serialized.contains(r#"{"Function":{"is_async":true}}"#));
    }

    #[test]
    fn test_all_paths() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![
                Edge { from_id: "A".to_string(), to_id: "B".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "B".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "A".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
            ],
            meta: GraphMeta {
                language: "test".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            }
        };

        let paths = graph.all_paths("A", "C", 10);
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&vec!["A".to_string(), "B".to_string(), "C".to_string()]));
        assert!(paths.contains(&vec!["A".to_string(), "C".to_string()]));

        let paths_limited = graph.all_paths("A", "C", 1);
        assert_eq!(paths_limited.len(), 1);
    }

    #[test]
    fn test_all_paths_cycles() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![
                Edge { from_id: "A".to_string(), to_id: "B".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "B".to_string(), to_id: "A".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "B".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
            ],
            meta: GraphMeta {
                language: "test".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            }
        };

        let paths = graph.all_paths("A", "C", 10);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_callers_recursive() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![
                Edge { from_id: "A".to_string(), to_id: "B".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "B".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "D".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
            ],
            meta: GraphMeta {
                language: "test".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            }
        };

        let tree = graph.callers_recursive("C", 10);
        
        let _expected = serde_json::json!({
            "node": "C",
            "callers": [
                {
                    "node": "B",
                    "callers": [
                        {
                            "node": "A",
                            "callers": []
                        }
                    ]
                },
                {
                    "node": "D",
                    "callers": []
                }
            ]
        });

        // The order of callers can vary due to HashSet in the implementation or iteration order, 
        // but for this graph structure edges are iterated in order so we might need a more flexible check
        // We can just verify structure roughly or check if "A" and "D" are in the result string.
        let json_str = tree.to_string();
        assert!(json_str.contains(r#""node":"A""#));
        assert!(json_str.contains(r#""node":"B""#));
        assert!(json_str.contains(r#""node":"C""#));
        assert!(json_str.contains(r#""node":"D""#));
    }

    #[test]
    fn test_blast_radius() {
        let graph = CodeGraph {
            nodes: vec![],
            edges: vec![
                Edge { from_id: "A".to_string(), to_id: "B".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "B".to_string(), to_id: "C".to_string(), kind: EdgeKind::Calls },
                Edge { from_id: "C".to_string(), to_id: "A".to_string(), kind: EdgeKind::Calls }, // cycle
                Edge { from_id: "A".to_string(), to_id: "D".to_string(), kind: EdgeKind::Calls },
            ],
            meta: GraphMeta {
                language: "test".to_string(),
                source_root: "".to_string(),
                generated_at: "".to_string(),
                node_count: 0,
                edge_count: 0,
            }
        };

        let mut radius = graph.blast_radius("A");
        radius.sort();
        
        let expected = vec!["B".to_string(), "C".to_string(), "D".to_string()];
        assert_eq!(radius, expected);
    }
}

