use crate::{CodeGraph, Edge, Node};
use serde::{Deserialize, Serialize};

/// Represents the differences between two `CodeGraph`s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphDiff {
    /// Nodes added in the head graph.
    pub added_nodes: Vec<Node>,
    /// Nodes removed in the head graph.
    pub removed_nodes: Vec<Node>,
    /// Edges added in the head graph.
    pub added_edges: Vec<Edge>,
    /// Edges removed in the head graph.
    pub removed_edges: Vec<Edge>,
}

/// Computes the difference between a base and head `CodeGraph`.
pub fn diff(base_graph: &CodeGraph, head_graph: &CodeGraph) -> GraphDiff {
    let mut added_nodes = Vec::new();
    let mut removed_nodes = Vec::new();
    let mut added_edges = Vec::new();
    let mut removed_edges = Vec::new();

    // Compute node diffs
    let base_node_ids: std::collections::HashSet<&str> =
        base_graph.nodes.iter().map(|n| n.id.as_str()).collect();
    let head_node_ids: std::collections::HashSet<&str> =
        head_graph.nodes.iter().map(|n| n.id.as_str()).collect();

    for node in &head_graph.nodes {
        if !base_node_ids.contains(node.id.as_str()) {
            added_nodes.push(node.clone());
        }
    }

    for node in &base_graph.nodes {
        if !head_node_ids.contains(node.id.as_str()) {
            removed_nodes.push(node.clone());
        }
    }

    // Compute edge diffs
    let base_edges: std::collections::HashSet<(&str, &str, &crate::EdgeKind)> = base_graph
        .edges
        .iter()
        .map(|e| (e.from_id.as_str(), e.to_id.as_str(), &e.kind))
        .collect();

    let head_edges: std::collections::HashSet<(&str, &str, &crate::EdgeKind)> = head_graph
        .edges
        .iter()
        .map(|e| (e.from_id.as_str(), e.to_id.as_str(), &e.kind))
        .collect();

    for edge in &head_graph.edges {
        let edge_tuple = (edge.from_id.as_str(), edge.to_id.as_str(), &edge.kind);
        if !base_edges.contains(&edge_tuple) {
            added_edges.push(edge.clone());
        }
    }

    for edge in &base_graph.edges {
        let edge_tuple = (edge.from_id.as_str(), edge.to_id.as_str(), &edge.kind);
        if !head_edges.contains(&edge_tuple) {
            removed_edges.push(edge.clone());
        }
    }

    GraphDiff {
        added_nodes,
        removed_nodes,
        added_edges,
        removed_edges,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EdgeKind, GraphMeta, NodeKind};

    fn empty_meta() -> GraphMeta {
        GraphMeta {
            language: "test".to_string(),
            source_root: "".to_string(),
            generated_at: "".to_string(),
            node_count: 0,
            edge_count: 0,
        }
    }

    #[test]
    fn test_diff_identical() {
        let mut graph = CodeGraph::new(empty_meta());
        graph.nodes.push(Node {
            id: "node1".to_string(),
            label: "Node 1".to_string(),
            kind: NodeKind::File,
            file_path: "node1.rs".to_string(),
            line: None,
            is_public: true,
        });

        let diff_result = diff(&graph, &graph);
        assert!(diff_result.added_nodes.is_empty());
        assert!(diff_result.removed_nodes.is_empty());
        assert!(diff_result.added_edges.is_empty());
        assert!(diff_result.removed_edges.is_empty());
    }

    #[test]
    fn test_diff_added_node() {
        let base = CodeGraph::new(empty_meta());
        let mut head = CodeGraph::new(empty_meta());

        let node = Node {
            id: "node1".to_string(),
            label: "Node 1".to_string(),
            kind: NodeKind::File,
            file_path: "node1.rs".to_string(),
            line: None,
            is_public: true,
        };
        head.nodes.push(node.clone());

        let diff_result = diff(&base, &head);
        assert_eq!(diff_result.added_nodes.len(), 1);
        assert_eq!(diff_result.added_nodes[0].id, "node1");
        assert!(diff_result.removed_nodes.is_empty());
    }

    #[test]
    fn test_diff_removed_node() {
        let mut base = CodeGraph::new(empty_meta());
        let head = CodeGraph::new(empty_meta());

        let node = Node {
            id: "node1".to_string(),
            label: "Node 1".to_string(),
            kind: NodeKind::File,
            file_path: "node1.rs".to_string(),
            line: None,
            is_public: true,
        };
        base.nodes.push(node.clone());

        let diff_result = diff(&base, &head);
        assert_eq!(diff_result.removed_nodes.len(), 1);
        assert_eq!(diff_result.removed_nodes[0].id, "node1");
        assert!(diff_result.added_nodes.is_empty());
    }

    #[test]
    fn test_diff_edges() {
        let mut base = CodeGraph::new(empty_meta());
        let mut head = CodeGraph::new(empty_meta());

        let edge_removed = Edge {
            from_id: "A".to_string(),
            to_id: "B".to_string(),
            kind: EdgeKind::Imports,
        };
        let edge_added = Edge {
            from_id: "A".to_string(),
            to_id: "C".to_string(),
            kind: EdgeKind::Imports,
        };

        base.edges.push(edge_removed.clone());
        head.edges.push(edge_added.clone());

        let diff_result = diff(&base, &head);

        assert_eq!(diff_result.removed_edges.len(), 1);
        assert_eq!(diff_result.removed_edges[0].to_id, "B");

        assert_eq!(diff_result.added_edges.len(), 1);
        assert_eq!(diff_result.added_edges[0].to_id, "C");
    }
}
