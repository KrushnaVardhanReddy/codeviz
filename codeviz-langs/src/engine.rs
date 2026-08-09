use crate::config::LangConfig;
use crate::grammar_map::get_language;
use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use tree_sitter::{Parser, Query, QueryCursor};

/// Generic Parser that executes tree-sitter queries defined in a `LangConfig`
/// to extract architecture nodes and edges from source code.
pub struct GenericParser {
    config: LangConfig,
    extensions: Vec<String>,
}

impl GenericParser {
    /// Create a new `GenericParser` from a loaded `LangConfig`.
    pub fn new(config: LangConfig) -> Self {
        let extensions = config.language.extensions.clone();
        Self { config, extensions }
    }

    fn run_query(
        &self,
        query_str: &str,
        language: tree_sitter::Language,
        source: &str,
        file_path: &str,
        process_match: &mut dyn FnMut(&tree_sitter::QueryMatch, &Query) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let query = Query::new(&language, query_str).map_err(|e| ParseError {
            message: format!("Invalid query string: {}", e),
            file_path: file_path.to_string(),
            line: None,
        })?;

        let mut parser = Parser::new();
        if parser.set_language(&language).is_err() {
            return Err(ParseError {
                message: "Failed to set language for tree-sitter".to_string(),
                file_path: file_path.to_string(),
                line: None,
            });
        }

        let tree = match parser.parse(source, None) {
            Some(t) => t,
            None => {
                return Err(ParseError {
                    message: "Failed to parse source code".to_string(),
                    file_path: file_path.to_string(),
                    line: None,
                })
            }
        };

        let mut cursor = QueryCursor::new();
        let source_bytes = source.as_bytes();
        let matches = cursor.matches(&query, tree.root_node(), source_bytes);

        for m in matches {
            process_match(&m, &query)?;
        }

        Ok(())
    }
}

impl LanguageParser for GenericParser {
    fn language_name(&self) -> &str {
        &self.config.language.name
    }

    fn supported_extensions(&self) -> &[&str] {
        // Since `supported_extensions` returns `&[&str]`, we can't easily return a reference
        // to a locally created vector of `&str`. However, we can leak it if we construct it once,
        // or we could just use Box::leak. Let's create it on the fly and leak it since
        // the parsers are typically long-lived.
        let ext_refs: Vec<&str> = self.extensions.iter().map(|s| s.as_str()).collect();
        let slice = ext_refs.into_boxed_slice();
        Box::leak(slice)
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let language = get_language(&self.config.language.grammar)?;

        let mut graph = CodeGraph::new(GraphMeta {
            language: self.language_name().to_string(),
            source_root: "".to_string(),
            generated_at: "".to_string(),
            node_count: 0,
            edge_count: 0,
        });

        // Add the file node
        graph.nodes.push(Node {
            id: file_path.to_string(),
            label: std::path::Path::new(file_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            kind: NodeKind::File,
            file_path: file_path.to_string(),
            line: Some(1),
            is_public: true,
        });

        let source_bytes = source.as_bytes();

        // Process functions
        if let Some(ref q_str) = self.config.queries.functions {
            self.run_query(
                q_str,
                language.clone(),
                source,
                file_path,
                &mut |m, query| {
                    let name_idx = query.capture_index_for_name("name");
                    let node_idx = query.capture_index_for_name("node");

                    let mut name_val = None;
                    let mut line_val = None;

                    for cap in m.captures {
                        if let Some(idx) = name_idx {
                            if cap.index == idx {
                                if let Ok(text) = cap.node.utf8_text(source_bytes) {
                                    name_val = Some(text.to_string());
                                }
                            }
                        }
                        if let Some(idx) = node_idx {
                            if cap.index == idx {
                                line_val = Some(cap.node.start_position().row as u32 + 1);
                            }
                        }
                    }

                    if let Some(name) = name_val {
                        graph.nodes.push(Node {
                            id: format!("{}::{}", file_path, name),
                            label: name,
                            kind: NodeKind::Function { is_async: false },
                            file_path: file_path.to_string(),
                            line: line_val,
                            is_public: true,
                        });
                    }
                    Ok(())
                },
            )?;
        }

        // Process classes
        if let Some(ref q_str) = self.config.queries.classes {
            self.run_query(
                q_str,
                language.clone(),
                source,
                file_path,
                &mut |m, query| {
                    let name_idx = query.capture_index_for_name("name");
                    let node_idx = query.capture_index_for_name("node");

                    let mut name_val = None;
                    let mut line_val = None;

                    for cap in m.captures {
                        if let Some(idx) = name_idx {
                            if cap.index == idx {
                                if let Ok(text) = cap.node.utf8_text(source_bytes) {
                                    name_val = Some(text.to_string());
                                }
                            }
                        }
                        if let Some(idx) = node_idx {
                            if cap.index == idx {
                                line_val = Some(cap.node.start_position().row as u32 + 1);
                            }
                        }
                    }

                    if let Some(name) = name_val {
                        graph.nodes.push(Node {
                            id: format!("{}::{}", file_path, name),
                            label: name,
                            kind: NodeKind::Class,
                            file_path: file_path.to_string(),
                            line: line_val,
                            is_public: true,
                        });
                    }
                    Ok(())
                },
            )?;
        }

        // Process imports
        if let Some(ref q_str) = self.config.queries.imports {
            self.run_query(
                q_str,
                language.clone(),
                source,
                file_path,
                &mut |m, query| {
                    let path_idx = query.capture_index_for_name("path");

                    for cap in m.captures {
                        if let Some(idx) = path_idx {
                            if cap.index == idx {
                                if let Ok(text) = cap.node.utf8_text(source_bytes) {
                                    // For basic imports, just record the edge to the imported path
                                    graph.edges.push(Edge {
                                        from_id: file_path.to_string(),
                                        to_id: text.to_string(),
                                        kind: EdgeKind::Imports,
                                    });
                                }
                            }
                        }
                    }
                    Ok(())
                },
            )?;
        }

        graph.meta.node_count = graph.nodes.len();
        graph.meta.edge_count = graph.edges.len();

        Ok(graph)
    }
}
