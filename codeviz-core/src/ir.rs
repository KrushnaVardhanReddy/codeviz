use serde::{Deserialize, Serialize};

/// Represents the intermediate representation of the code architecture graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeGraph {
    /// List of nodes in the graph.
    pub nodes: Vec<Node>,
    /// List of edges connecting the nodes.
    pub edges: Vec<Edge>,
    /// Metadata about the graph.
    pub meta: GraphMeta,
}


/// Statistics for a CodeGraph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphStats {
    /// Total number of nodes in the graph.
    pub total_nodes: usize,
    /// Total number of edges in the graph.
    pub total_edges: usize,
    /// List of languages in the graph.
    pub languages: Vec<String>,
    /// List of entry point node labels.
    pub entry_points: Vec<String>,
    /// List of most imported module labels.
    pub top_modules: Vec<String>,
    /// Number of circular dependencies detected.
    pub circular_dep_count: usize,
}

impl CodeGraph {
    /// Creates a new empty CodeGraph.
    pub fn new(meta: GraphMeta) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            meta,
        }
    }

    /// Generates a human-readable summary and statistics of the architecture.
    pub fn summarize(&self) -> (String, GraphStats) {
        use std::collections::{HashMap, HashSet};

        let total_nodes = self.nodes.len();
        let total_edges = self.edges.len();

        let mut languages = Vec::new();
        languages.push(self.meta.language.clone());

        let mut file_paths = HashSet::new();
        for node in &self.nodes {
            file_paths.insert(&node.file_path);
        }
        let file_count = file_paths.len();

        let mut has_incoming_calls = HashSet::new();
        let mut import_counts: HashMap<&String, usize> = HashMap::new();

        for edge in &self.edges {
            if edge.kind == EdgeKind::Calls {
                has_incoming_calls.insert(&edge.to_id);
            }
            if edge.kind == EdgeKind::Imports {
                *import_counts.entry(&edge.to_id).or_insert(0) += 1;
            }
        }

        let mut entry_point_labels: Vec<String> = self.nodes.iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }) && !has_incoming_calls.contains(&n.id))
            .map(|n| n.label.clone())
            .collect();

        entry_point_labels.sort();
        let entry_points_stats = entry_point_labels.clone();

        let mut entry_points_str = "None".to_string();
        if !entry_point_labels.is_empty() {
            if entry_point_labels.len() > 10 {
                entry_point_labels.truncate(10);
                entry_point_labels.push("...".to_string());
            }
            entry_points_str = entry_point_labels.join(", ");
        }

        let mut id_to_label: HashMap<&String, &String> = HashMap::new();
        for node in &self.nodes {
            id_to_label.insert(&node.id, &node.label);
        }

        let mut module_imports: Vec<(&String, usize)> = import_counts.into_iter().collect();
        module_imports.sort_by(|a, b| {
            b.1.cmp(&a.1).then_with(|| {
                let a_label = id_to_label.get(a.0);
                let b_label = id_to_label.get(b.0);
                a_label.cmp(&b_label)
            })
        });

        let mut top_module_labels = Vec::new();
        for (id, _) in module_imports.iter().take(5) {
            if let Some(node) = self.nodes.iter().find(|n| &n.id == *id) {
                top_module_labels.push(node.label.clone());
            }
        }

        let top_modules_stats = top_module_labels.clone();

        let mut top_modules_str = "None".to_string();
        if !top_module_labels.is_empty() {
            top_modules_str = top_module_labels.join(", ");
        }

        // Tarjan's for circular dependencies (SCCs)
        let mut adj: HashMap<&String, Vec<&String>> = HashMap::new();
        for edge in &self.edges {
            // Include Imports and Calls for circular deps? The spec doesn't specify which edges. Let's use all edges or just Imports?
            // Usually circular deps means imports, but we can just use all edges since it's a general graph.
            adj.entry(&edge.from_id).or_default().push(&edge.to_id);
        }

        let mut index = 0;
        let mut indices: HashMap<&String, usize> = HashMap::new();
        let mut lowlinks: HashMap<&String, usize> = HashMap::new();
        let mut on_stack: HashSet<&String> = HashSet::new();
        let mut stack: Vec<&String> = Vec::new();
        let mut scc_count = 0;

        // Helper function inside summarize is tricky in Rust without Rc/RefCell for mutation,
        // Let's just use iterative or simple recursive with passed mutable state.
        #[allow(clippy::too_many_arguments)]
        fn strongconnect<'a>(
            v: &'a String,
            index: &mut usize,
            indices: &mut HashMap<&'a String, usize>,
            lowlinks: &mut HashMap<&'a String, usize>,
            on_stack: &mut HashSet<&'a String>,
            stack: &mut Vec<&'a String>,
            adj: &HashMap<&'a String, Vec<&'a String>>,
            scc_count: &mut usize,
        ) {
            indices.insert(v, *index);
            lowlinks.insert(v, *index);
            *index += 1;
            stack.push(v);
            on_stack.insert(v);

            if let Some(neighbors) = adj.get(v) {
                for w in neighbors {
                    if !indices.contains_key(w) {
                        strongconnect(w, index, indices, lowlinks, on_stack, stack, adj, scc_count);
                        let min = std::cmp::min(lowlinks[v], lowlinks[w]);
                        lowlinks.insert(v, min);
                    } else if on_stack.contains(w) {
                        let min = std::cmp::min(lowlinks[v], indices[w]);
                        lowlinks.insert(v, min);
                    }
                }
            }

            if lowlinks[v] == indices[v] {
                let mut component = Vec::new();
                while let Some(w) = stack.pop() {
                    on_stack.remove(w);
                    component.push(w);
                    if w == v {
                        break;
                    }
                }
                if component.len() > 1 {
                    *scc_count += 1;
                }
            }
        }

        for node in &self.nodes {
            let id = &node.id;
            if !indices.contains_key(id) {
                strongconnect(id, &mut index, &mut indices, &mut lowlinks, &mut on_stack, &mut stack, &adj, &mut scc_count);
            }
        }

        let language_list = languages.join(", ");

        let mut summary = format!(
            "This is a {} codebase with {} symbols across {} files.

Entry points: {}.
Most-imported modules: {}.",
            language_list, total_nodes, file_count, entry_points_str, top_modules_str
        );

        if scc_count > 0 {
            summary.push_str(&format!("
⚠️ {} circular dependencies detected.", scc_count));
        }

        let stats = GraphStats {
            total_nodes,
            total_edges,
            languages,
            entry_points: entry_points_stats,
            top_modules: top_modules_stats,
            circular_dep_count: scc_count,
        };

        (summary, stats)
    }
}
/// A node in the code graph, representing a code symbol or file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    /// Globally unique ID: "{file_path}::{symbol_name}"
    pub id: String,
    /// Display name, stripped of generics/lifetimes
    pub label: String,
    /// Type of node
    pub kind: NodeKind,
    /// Path relative to source_root
    pub file_path: String,
    /// 1-indexed line number; None if not resolvable
    pub line: Option<u32>,
    /// true if exported/pub
    pub is_public: bool,
}

/// The kind of a code graph node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeKind {
    /// A file
    File,
    /// A module
    Module,
    /// A function
    Function {
        /// True if the function is async
        is_async: bool,
    },
    /// A class
    Class,
    /// An interface
    Interface,
    /// A constant
    Constant,
}

/// An edge in the code graph, representing a relationship between two nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Edge {
    /// ID of the source node
    pub from_id: String,
    /// ID of the target node (may be external)
    pub to_id: String,
    /// Type of edge relationship
    pub kind: EdgeKind,
}

/// The kind of a code graph edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EdgeKind {
    /// Module-level dependency
    Imports,
    /// Function invokes function
    Calls,
    /// Class extends class / trait extends trait
    Inherits,
    /// Class implements interface / impl Trait for Struct
    Implements,
    /// Function returns type
    Returns,
    /// Function creates instance of class
    Instantiates,
}

/// Metadata about the code graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphMeta {
    /// Language name (e.g., "python", "typescript")
    pub language: String,
    /// Absolute path scanned
    pub source_root: String,
    /// ISO 8601 UTC timestamp of generation
    pub generated_at: String,
    /// Total number of nodes
    pub node_count: usize,
    /// Total number of edges
    pub edge_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let mut graph = CodeGraph::new(GraphMeta {
            language: "rust".to_string(),
            source_root: "/test".to_string(),
            generated_at: "now".to_string(),
            node_count: 0,
            edge_count: 0,
        });

        graph.nodes.push(Node {
            id: "src/main.rs::main".to_string(),
            label: "main".to_string(),
            kind: NodeKind::Function { is_async: false },
            file_path: "src/main.rs".to_string(),
            line: Some(1),
            is_public: true,
        });

        graph.nodes.push(Node {
            id: "src/lib.rs::lib_func".to_string(),
            label: "lib_func".to_string(),
            kind: NodeKind::Function { is_async: false },
            file_path: "src/lib.rs".to_string(),
            line: Some(1),
            is_public: true,
        });

        // Add a circular dependency
        graph.edges.push(Edge {
            from_id: "src/main.rs::main".to_string(),
            to_id: "src/lib.rs::lib_func".to_string(),
            kind: EdgeKind::Calls,
        });
        graph.edges.push(Edge {
            from_id: "src/lib.rs::lib_func".to_string(),
            to_id: "src/main.rs::main".to_string(),
            kind: EdgeKind::Calls,
        });

        // Add an import to test top modules
        graph.edges.push(Edge {
            from_id: "src/main.rs::main".to_string(),
            to_id: "src/lib.rs::lib_func".to_string(),
            kind: EdgeKind::Imports,
        });

        graph.edges.push(Edge {
            from_id: "src/main.rs::main".to_string(),
            to_id: "src/lib.rs::lib_func".to_string(),
            kind: EdgeKind::Imports,
        });

        let (summary, stats) = graph.summarize();

        assert_eq!(stats.total_nodes, 2);
        assert_eq!(stats.total_edges, 4);
        assert_eq!(stats.languages, vec!["rust".to_string()]);
        assert_eq!(stats.circular_dep_count, 1);
        assert!(summary.contains("⚠️ 1 circular dependencies detected."));
        assert!(summary.contains("Most-imported modules: lib_func."));
        assert!(summary.contains("Entry points: None.")); // both have incoming calls
        assert!(summary.contains("2 symbols across 2 files."));
    }
}
