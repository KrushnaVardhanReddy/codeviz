use crate::CodeGraph;

/// Renders the given `CodeGraph` into a DOT digraph string.
pub fn render_dot(graph: &CodeGraph) -> String {
    let mut output = String::new();
    output.push_str("digraph G {\n");

    let mut valid_nodes = std::collections::HashSet::new();

    for node in &graph.nodes {
        valid_nodes.insert(node.id.as_str());
        let id = sanitize_id(&node.id);
        let label = &node.label;
        output.push_str(&format!("    {} [label=\"{}\"];\n", id, label));
    }

    for edge in &graph.edges {
        if !valid_nodes.contains(edge.from_id.as_str())
            || !valid_nodes.contains(edge.to_id.as_str())
        {
            continue;
        }

        let from = sanitize_id(&edge.from_id);
        let to = sanitize_id(&edge.to_id);
        output.push_str(&format!("    {} -> {};\n", from, to));
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
