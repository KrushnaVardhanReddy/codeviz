use crate::{CodeGraph, ir::NodeKind};

/// Renders the given `CodeGraph` into a DOT digraph string.
pub fn render_dot(graph: &CodeGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph codeviz {\n");
    output.push_str("    rankdir=TD;\n");

    let mut valid_nodes = std::collections::HashSet::new();

    for node in &graph.nodes {
        valid_nodes.insert(node.id.as_str());
        let id = sanitize_id(&node.id);
        let label = &node.label;

        let shape = match node.kind {
            NodeKind::Function { .. } => "ellipse",
            NodeKind::Class => "box",
            NodeKind::Interface => "diamond",
            NodeKind::Module | NodeKind::File => "folder",
            NodeKind::Constant => "box", // Defaulting to box
        };

        output.push_str(&format!(
            "    \"{}\" [label=\"{}\" shape={}];\n",
            id, label, shape
        ));
    }

    for edge in &graph.edges {
        if !valid_nodes.contains(edge.from_id.as_str())
            || !valid_nodes.contains(edge.to_id.as_str())
        {
            continue;
        }

        let from = sanitize_id(&edge.from_id);
        let to = sanitize_id(&edge.to_id);
        let edge_label = match edge.kind {
            crate::ir::EdgeKind::Imports => "Imports",
            crate::ir::EdgeKind::Calls => "Calls",
            crate::ir::EdgeKind::Inherits => "Inherits",
            crate::ir::EdgeKind::Implements => "Implements",
            crate::ir::EdgeKind::Returns => "Returns",
            crate::ir::EdgeKind::Instantiates => "Instantiates",
            crate::ir::EdgeKind::Contains => "Contains",
        };

        output.push_str(&format!(
            "    \"{}\" -> \"{}\" [label=\"{}\"];\n",
            from, to, edge_label
        ));
    }

    output.push_str("}\n");
    output
}

pub(crate) fn sanitize_id(id: &str) -> String {
    id.replace("/", "_")
        .replace(".", "_")
        .replace("::", "_")
        .replace("-", "_")
}
