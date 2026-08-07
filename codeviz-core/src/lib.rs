use serde::{Deserialize, Serialize};

pub mod render;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub meta: GraphMeta,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub kind: NodeKind,
    pub file_path: String,
    pub line: Option<u32>,
    pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    File,
    Module,
    Function { is_async: bool },
    Class,
    Interface,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub from_id: String,
    pub to_id: String,
    pub kind: EdgeKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EdgeKind {
    Imports,
    Calls,
    Inherits,
    Implements,
    Returns,
    Instantiates,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphMeta {
    pub language: String,
    pub source_root: String,
    pub generated_at: String,
    pub node_count: usize,
    pub edge_count: usize,
}
