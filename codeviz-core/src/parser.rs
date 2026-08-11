use crate::ir::CodeGraph;

/// An error that occurred during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    /// A descriptive error message.
    pub message: String,
    /// The path of the file being parsed.
    pub file_path: String,
    /// The line number where the error occurred, if applicable.
    pub line: Option<u32>,
}

/// Interface that every language adapter must implement to parse code into a CodeGraph.
pub trait LanguageParser: Send + Sync {
    /// Human-readable name, e.g. "python", "typescript"
    fn language_name(&self) -> &str;

    /// File extensions this parser handles, e.g. ["py"]
    fn supported_extensions(&self) -> &[&str];

    /// Parse source code string into a CodeGraph.
    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError>;
}

/// A registry of available language parsers, handling dispatch by file extension.
pub struct LanguageRegistry {
    parsers: Vec<Box<dyn LanguageParser>>,
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageRegistry {
    /// Creates a new, empty LanguageRegistry.
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
        }
    }

    /// Registers a new parser. If multiple parsers claim the same extension,
    /// the last registered parser takes priority.
    pub fn register(&mut self, parser: Box<dyn LanguageParser>) {
        self.parsers.push(parser);
    }

    /// Returns the parser for a given file extension, if any.
    /// Returns the last registered parser that supports the extension.
    pub fn find_parser(&self, extension: &str) -> Option<&dyn LanguageParser> {
        self.parsers
            .iter()
            .rev()
            .find(|p| p.supported_extensions().contains(&extension))
            .map(|p| p.as_ref())
    }

    /// Dispatches parsing to the appropriate parser based on the file path's extension.
    /// Returns a `ParseError` if no suitable parser is found.
    pub fn parse_file(&self, file_path: &str, source: &str) -> Result<CodeGraph, ParseError> {
        let normalized_path = crate::path_utils::normalize_path(file_path);
        let extension = std::path::Path::new(&normalized_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match self.find_parser(extension) {
            Some(parser) => parser.parse(source, &normalized_path),
            None => Err(ParseError {
                message: format!("No parser for extension: {}", extension),
                file_path: normalized_path,
                line: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{GraphMeta, Node, NodeKind};

    #[test]
    fn test_node_ids_use_forward_slashes() {
        // Simulate a Windows-style path being passed to the parser
        let file_path = "src\\lib\\utils.py";
        let normalized = crate::path_utils::normalize_path(file_path);
        assert!(
            !normalized.contains('\\'),
            "Node paths must not contain backslashes"
        );
        assert_eq!(normalized, "src/lib/utils.py");
    }

    struct MockParser {
        name: &'static str,
        extensions: Vec<&'static str>,
    }

    impl LanguageParser for MockParser {
        fn language_name(&self) -> &str {
            self.name
        }

        fn supported_extensions(&self) -> &[&str] {
            &self.extensions
        }

        fn parse(&self, _source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
            let meta = GraphMeta {
                language: self.name.to_string(),
                source_root: "/".to_string(),
                generated_at: "2023-01-01T00:00:00Z".to_string(),
                node_count: 1,
                edge_count: 0,
            };
            let mut graph = CodeGraph::new(meta);
            graph.nodes.push(Node {
                id: format!("{}::mock", file_path),
                label: "mock".to_string(),
                kind: NodeKind::File,
                file_path: file_path.to_string(),
                line: None,
                is_public: true, parent_id: None,
            });
            Ok(graph)
        }
    }

    #[test]
    fn test_parse_file_dispatches_correctly() {
        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(MockParser {
            name: "python",
            extensions: vec!["py"],
        }));
        registry.register(Box::new(MockParser {
            name: "typescript",
            extensions: vec!["ts", "tsx"],
        }));

        let py_graph = registry.parse_file("test.py", "source").unwrap();
        assert_eq!(py_graph.meta.language, "python");
        assert_eq!(py_graph.nodes[0].id, "test.py::mock");

        let ts_graph = registry.parse_file("app.tsx", "source").unwrap();
        assert_eq!(ts_graph.meta.language, "typescript");
    }

    #[test]
    fn test_parse_file_unknown_extension() {
        let registry = LanguageRegistry::new();
        let result = registry.parse_file("test.unknown", "source");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "No parser for extension: unknown");
        assert_eq!(err.file_path, "test.unknown");
    }

    #[test]
    fn test_parser_precedence() {
        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(MockParser {
            name: "old_js",
            extensions: vec!["js"],
        }));
        registry.register(Box::new(MockParser {
            name: "new_js",
            extensions: vec!["js"],
        }));

        let graph = registry.parse_file("script.js", "source").unwrap();
        assert_eq!(graph.meta.language, "new_js");
    }

    #[test]
    fn test_integration_path_normalization() {
        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(MockParser {
            name: "python",
            extensions: vec!["py"],
        }));

        // Pass a Windows path
        let graph = registry
            .parse_file("src\\auth\\middleware.py", "source")
            .unwrap();

        // Assert the node file_path is normalized
        assert_eq!(graph.nodes[0].file_path, "src/auth/middleware.py");
        assert_eq!(graph.nodes[0].id, "src/auth/middleware.py::mock");
    }
}
