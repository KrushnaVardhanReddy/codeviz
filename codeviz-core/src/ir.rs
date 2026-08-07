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

impl CodeGraph {
    /// Creates a new empty CodeGraph.
    pub fn new(meta: GraphMeta) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            meta,
        }
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
