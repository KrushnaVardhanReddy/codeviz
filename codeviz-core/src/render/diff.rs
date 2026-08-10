use crate::diff::GraphDiff;

/// Renders a GraphDiff into a human-readable text format.
pub fn render_diff_human(
    diff: &GraphDiff,
    base_ref: &str,
    head_ref: &str,
) -> Result<String, String> {
    let mut out = String::new();
    out.push_str(&format!(
        "📊 Architecture diff: {} → {}\n",
        base_ref, head_ref
    ));

    out.push_str(&format!(
        "\n  ➕ Added nodes ({}):\n",
        diff.added_nodes.len()
    ));
    if diff.added_nodes.is_empty() {
        out.push_str("     none\n");
    } else {
        for node in &diff.added_nodes {
            out.push_str(&format!("     {}\n", node.id));
        }
    }

    out.push_str(&format!(
        "\n  ➖ Removed nodes ({}):\n",
        diff.removed_nodes.len()
    ));
    if diff.removed_nodes.is_empty() {
        out.push_str("     none\n");
    } else {
        for node in &diff.removed_nodes {
            out.push_str(&format!("     {}\n", node.id));
        }
    }

    out.push_str(&format!(
        "\n  ➕ Added edges ({}):\n",
        diff.added_edges.len()
    ));
    if diff.added_edges.is_empty() {
        out.push_str("     none\n");
    } else {
        for edge in &diff.added_edges {
            out.push_str(&format!(
                "     {} → {} [{:?}]\n",
                edge.from_id, edge.to_id, edge.kind
            ));
        }
    }

    out.push_str(&format!(
        "\n  ➖ Removed edges ({}):\n",
        diff.removed_edges.len()
    ));
    if diff.removed_edges.is_empty() {
        out.push_str("     none\n");
    } else {
        for edge in &diff.removed_edges {
            out.push_str(&format!(
                "     {} → {} [{:?}]\n",
                edge.from_id, edge.to_id, edge.kind
            ));
        }
    }

    Ok(out)
}

/// Renders a GraphDiff into a JSON string.
pub fn render_diff_json(diff: &GraphDiff) -> Result<String, String> {
    serde_json::to_string_pretty(diff).map_err(|e| format!("JSON error: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, EdgeKind, Node, NodeKind};

    #[test]
    fn test_render_diff_human_empty() {
        let diff = GraphDiff {
            added_nodes: vec![],
            removed_nodes: vec![],
            added_edges: vec![],
            removed_edges: vec![],
        };
        let out = render_diff_human(&diff, "main", "feature").unwrap();
        assert!(out.contains("📊 Architecture diff: main → feature"));
        assert!(out.contains("Added nodes (0):\n     none"));
    }

    #[test]
    fn test_render_diff_human_with_changes() {
        let diff = GraphDiff {
            added_nodes: vec![Node {
                id: "A".to_string(),
                label: "A".to_string(),
                kind: NodeKind::File,
                file_path: "A".to_string(),
                line: None,
                is_public: true,
            }],
            removed_nodes: vec![],
            added_edges: vec![Edge {
                from_id: "A".to_string(),
                to_id: "B".to_string(),
                kind: EdgeKind::Imports,
            }],
            removed_edges: vec![],
        };
        let out = render_diff_human(&diff, "main", "feature").unwrap();
        assert!(out.contains("Added nodes (1):\n     A"));
        assert!(out.contains("Added edges (1):\n     A → B [Imports]"));
    }

    #[test]
    fn test_render_diff_json() {
        let diff = GraphDiff {
            added_nodes: vec![],
            removed_nodes: vec![],
            added_edges: vec![],
            removed_edges: vec![],
        };
        let out = render_diff_json(&diff).unwrap();
        assert!(out.contains("\"added_nodes\": []"));
    }
}
