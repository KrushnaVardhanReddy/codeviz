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
    /// Control flow graphs for functions (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_flow: Option<Vec<ControlFlowGraph>>,
}

impl CodeGraph {
    /// Creates a new empty CodeGraph.
    pub fn new(meta: GraphMeta) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            control_flow: Vec::new(),
            meta,
            control_flow: None,
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

