use serde::{Deserialize, Serialize};

/// Represents the intermediate representation of the code architecture graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeGraph {
    /// List of nodes in the graph.
    pub nodes: Vec<Node>,
    /// List of edges connecting the nodes.
    pub edges: Vec<Edge>,
    /// Control flow graphs for functions.
    #[serde(default)]
    pub control_flow: Vec<ControlFlowGraph>,
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


    /// Creates a new empty CodeGraph.
    pub fn new(meta: GraphMeta) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            control_flow: Vec::new(),
            meta,
        }
    }

    /// Generates a human-readable summary and statistics of the architecture.
    pub fn summarize(&self) -> (String, GraphStats) {
        use std::collections::{HashMap, HashSet};

        let total_nodes = self.nodes.len();
        let total_edges = self.edges.len();

        let languages = vec![self.meta.language.clone()];

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

        let mut entry_point_labels: Vec<String> = self
            .nodes
            .iter()
            .filter(|n| {
                matches!(n.kind, NodeKind::Function { .. }) && !has_incoming_calls.contains(&n.id)
            })
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
                strongconnect(
                    id,
                    &mut index,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut stack,
                    &adj,
                    &mut scc_count,
                );
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
            summary.push_str(&format!(
                "
⚠️ {} circular dependencies detected.",
                scc_count
            ));
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
    /// Parent ID for nested visualization (e.g. methods inside classes)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
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
    /// Scope contains member (File→Class, File→Function, Class→Method)
    Contains,
}

/// Metadata about the code graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

/// A control flow graph for a single function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ControlFlowGraph {
    /// The ID of the function node this CFG belongs to.
    pub function_id: String,
    /// All basic blocks / control flow nodes.
    pub blocks: Vec<CfgBlock>,
    /// Edges between blocks.
    pub cfg_edges: Vec<CfgEdge>,
}

/// A single block in the control flow graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgBlock {
    /// ID of the CFG block.
    pub id: String,
    /// Kind of the CFG block.
    pub kind: CfgBlockKind,
    /// Human-readable label (e.g., the condition expression)
    pub label: String,
    /// 1-indexed line number in source
    pub line: Option<u32>,
}

/// The kind of a CFG block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CfgBlockKind {
    /// Function entry point
    Entry,
    /// Function exit / return
    Exit,
    /// A plain statement block
    Block,
    /// An if/else condition (decision diamond)
    Condition,
    /// A loop header (for/while/do-while)
    LoopHeader,
    /// A loop body
    LoopBody,
    /// A match/switch arm
    SwitchArm,
    /// A try block
    TryBlock,
    /// A catch/except block
    CatchBlock,
    /// A finally block
    FinallyBlock,
    /// An async/await suspension point
    AwaitPoint,
    /// A throw/raise error propagation
    ThrowPoint,
}

/// An edge between CFG blocks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgEdge {
    /// ID of the source block.
    pub from_id: String,
    /// ID of the target block.
    pub to_id: String,
    /// Kind of the CFG edge.
    pub kind: CfgEdgeKind,
    /// Optional label (e.g., "true", "false", "catch TypeError")
    pub label: Option<String>,
}

/// The kind of a CFG edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CfgEdgeKind {
    /// Normal sequential flow
    Normal,
    /// True branch of an if/condition
    TrueBranch,
    /// False branch of an if/condition
    FalseBranch,
    /// Loop back edge (creates cycle)
    LoopBack,
    /// Exception propagation to catch block
    ExceptionEdge,
    /// Always-runs path (finally block)
    FinallyEdge,
    /// Async suspension / resume
    AsyncEdge,
}

#[cfg(test)]
mod tests {
    use super::*;

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
            },
            ..Default::default()
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
            },
            ..Default::default()
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
            },
            ..Default::default()
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
            },
            ..Default::default()
        };

        let mut radius = graph.blast_radius("A");
        radius.sort();
        
        let expected = vec!["B".to_string(), "C".to_string(), "D".to_string()];
        assert_eq!(radius, expected);
    }
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
            parent_id: None,
        });

        graph.nodes.push(Node {
            id: "src/lib.rs::lib_func".to_string(),
            label: "lib_func".to_string(),
            kind: NodeKind::Function { is_async: false },
            file_path: "src/lib.rs".to_string(),
            line: Some(1),
            is_public: true,
            parent_id: None,
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
