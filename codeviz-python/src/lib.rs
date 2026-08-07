use codeviz_core::{CodeGraph, GraphMeta, LanguageParser, ParseError};

/// A basic Python parser implementation for integration purposes.
pub struct PythonParser;

impl PythonParser {
    /// Creates a new PythonParser.
    pub fn new() -> Self {
        Self
    }
}

impl Default for PythonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for PythonParser {
    fn language_name(&self) -> &str {
        "python"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["py"]
    }

    fn parse(&self, _source: &str, _file_path: &str) -> Result<CodeGraph, ParseError> {
        let meta = GraphMeta {
            language: "python".to_string(),
            source_root: "/".to_string(),
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            node_count: 0,
            edge_count: 0,
        };
        Ok(CodeGraph::new(meta))
    }
}
