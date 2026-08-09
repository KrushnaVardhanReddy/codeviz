use crate::CodeGraph;

/// Renders the given `CodeGraph` into a JSON string.
pub fn render_json(graph: &CodeGraph) -> Result<String, String> {
    serde_json::to_string_pretty(graph).map_err(|e| format!("Failed to serialize graph: {}", e))
}
