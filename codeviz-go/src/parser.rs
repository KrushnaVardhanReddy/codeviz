use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use codeviz_core::path_utils::normalize_path;
use std::path::Path;
use tree_sitter::{Node as TsNode, Parser, Tree};
use tree_sitter_go::language;

/// A parser for the Go programming language using tree-sitter.
pub struct GoParser;

impl GoParser {
    /// Creates a new `GoParser`.
    pub fn new() -> Self {
        Self
    }

    fn traverse_tree(
        &self,
        tree: &Tree,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let source_root = if graph.meta.source_root.is_empty() {
            std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            graph.meta.source_root.clone()
        };

        let root = tree.root_node();
        let mut cursor = root.walk();

        for child in root.children(&mut cursor) {
            // Tree-sitter handles `//go:build ...` as comments.
            // By default, we ignore comments because they don't match any of the node kinds we care about.
            // Just to be compliant with the spec "Skip Silently: Build tag lines",
            // we ensure we don't accidentally process them.
            if child.kind() == "comment" {
                continue; // Skip build tags and other comments
            }

            match child.kind() {
                "import_declaration" => {
                    self.extract_import(child, source_bytes, file_path, graph, &source_root)?;
                }
                "type_declaration" => {
                    self.extract_type(child, source_bytes, file_path, graph)?;
                }
                "function_declaration" => {
                    self.extract_function(child, source_bytes, file_path, graph)?;
                }
                "method_declaration" => {
                    self.extract_method(child, source_bytes, file_path, graph)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn extract_import(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        source_root: &str,
    ) -> Result<(), ParseError> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "import_spec" {
                self.extract_import_spec(child, source_bytes, file_path, graph, source_root)?;
            } else if child.kind() == "import_spec_list" {
                let mut list_cursor = child.walk();
                for spec in child.children(&mut list_cursor) {
                    if spec.kind() == "import_spec" {
                        self.extract_import_spec(
                            spec,
                            source_bytes,
                            file_path,
                            graph,
                            source_root,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_import_spec(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        source_root: &str,
    ) -> Result<(), ParseError> {
        // Skip blank imports
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                if name == "_" {
                    return Ok(());
                }
            }
        }

        // Find the "path" field or the literal child.
        if let Some(path_node) = node.child_by_field_name("path") {
            if let Ok(path_str) = path_node.utf8_text(source_bytes) {
                // path_str is like "fmt" with quotes. Remove the quotes.
                let mut clean_path = path_str.trim_matches('"').to_string();
                if clean_path != "C" {
                    // skip cgo
                    if !source_root.is_empty() {
                        clean_path = resolve_import_path(&clean_path, source_root);
                    }
                    graph.edges.push(Edge {
                        from_id: file_path.to_string(),
                        to_id: clean_path,
                        kind: EdgeKind::Imports,
                    });
                }
            }
        }
        Ok(())
    }

    fn extract_type(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_spec" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    if let Ok(name) = name_node.utf8_text(source_bytes) {
                        if let Some(type_node) = child.child_by_field_name("type") {
                            let kind = match type_node.kind() {
                                "struct_type" => NodeKind::Class,
                                "interface_type" => NodeKind::Interface,
                                _ => continue, // We only care about structs and interfaces based on the spec
                            };

                            let id = format!("{}::{}", normalize_path(file_path), name);
                            graph.nodes.push(Node {
                                id: id.clone(),
                                label: name.to_string(),
                                kind: kind.clone(),
                                file_path: normalize_path(file_path),
                                line: Some(name_node.start_position().row as u32 + 1),
                                is_public: name
                                    .chars()
                                    .next()
                                    .map(|c| c.is_uppercase())
                                    .unwrap_or(false),
                parent_id: None,
                            });

                            if kind == NodeKind::Class {
                                // Extract embedded structs as Inherits edges
                                let mut struct_cursor = type_node.walk();
                                for field_list_candidate in type_node.children(&mut struct_cursor) {
                                    if field_list_candidate.kind() == "field_declaration_list" {
                                        let mut field_cursor = field_list_candidate.walk();
                                        for field_decl in
                                            field_list_candidate.children(&mut field_cursor)
                                        {
                                            if field_decl.kind() == "field_declaration" {
                                                // An embedded struct field lacks a 'name' field
                                                if field_decl.child_by_field_name("name").is_none()
                                                {
                                                    if let Some(type_ident) =
                                                        field_decl.child_by_field_name("type")
                                                    {
                                                        if let Ok(embedded_name) =
                                                            type_ident.utf8_text(source_bytes)
                                                        {
                                                            // In a real scenario we'd resolve it, here we assume it's in the same file or package
                                                            graph.edges.push(Edge {
                                                                from_id: id.clone(),
                                                                to_id: format!(
                                                                    "{}::{}",
                                                                    normalize_path(file_path),
                                                                    embedded_name
                                                                ),
                                                                kind: EdgeKind::Inherits,
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_function(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", normalize_path(file_path), name);
                let is_public = name == "main"
                    || name
                        .chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false);

                graph.nodes.push(Node { parent_id: None, 
                    id,
                    label: name.to_string(),
                    kind: NodeKind::Function { is_async: false },
                    file_path: normalize_path(file_path),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public,
                });
            }
        }
        Ok(())
    }

    fn extract_method(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let mut receiver_name = "";
                if let Some(receiver) = node.child_by_field_name("receiver") {
                    let mut rec_cursor = receiver.walk();
                    for child in receiver.children(&mut rec_cursor) {
                        if child.kind() == "parameter_declaration" {
                            if let Some(type_node) = child.child_by_field_name("type") {
                                // could be pointer_type or type_identifier
                                if type_node.kind() == "pointer_type" {
                                    let mut ptr_cursor = type_node.walk();
                                    for t_child in type_node.children(&mut ptr_cursor) {
                                        if t_child.kind() == "type_identifier" {
                                            if let Ok(t_name) = t_child.utf8_text(source_bytes) {
                                                receiver_name = t_name;
                                            }
                                        }
                                    }
                                } else if type_node.kind() == "type_identifier" {
                                    if let Ok(t_name) = type_node.utf8_text(source_bytes) {
                                        receiver_name = t_name;
                                    }
                                }
                            }
                        }
                    }
                }

                let id = if receiver_name.is_empty() {
                    format!("{}::{}", normalize_path(file_path), name)
                } else {
                    format!("{}::{}::{}", normalize_path(file_path), receiver_name, name)
                    // or just use name for label
                };

                let is_public = name
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false);

                graph.nodes.push(Node { parent_id: None, 
                    id,
                    label: name.to_string(),
                    kind: NodeKind::Function { is_async: false },
                    file_path: normalize_path(file_path),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public,
                });
            }
        }
        Ok(())
    }
}

pub(crate) fn resolve_import_path(import_path: &str, source_root: &str) -> String {
    let mod_path = Path::new(source_root).join("go.mod");
    if let Ok(content) = std::fs::read_to_string(mod_path) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("module ") {
                let module_name = line.trim_start_matches("module ").trim();
                if import_path.starts_with(module_name) {
                    // Normalizing to relative to source root
                    if import_path == module_name {
                        return "".to_string(); // not realistic but just in case
                    }
                    return import_path
                        .strip_prefix(module_name)
                        .unwrap_or("")
                        .trim_start_matches('/')
                        .to_string();
                }
            }
        }
    }
    import_path.to_string()
}

impl Default for GoParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for GoParser {
    fn language_name(&self) -> &str {
        "go"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["go"]
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();
        if parser.set_language(&language()).is_err() {
            return Err(ParseError {
                message: "Failed to set Go language for tree-sitter".to_string(),
                file_path: file_path.to_string(),
                line: None,
            });
        }

        let tree = match parser.parse(source, None) {
            Some(tree) => tree,
            None => {
                return Err(ParseError {
                    message: "Failed to parse source code".to_string(),
                    file_path: file_path.to_string(),
                    line: None,
                });
            }
        };

        let mut graph = CodeGraph::new(GraphMeta {
            language: self.language_name().to_string(),
            source_root: "".to_string(), // Caller should modify this later if needed
            generated_at: "".to_string(),
            node_count: 0,
            edge_count: 0,
        });

        // Add the file node itself as the "module" node for this file
        graph.nodes.push(Node {
            id: normalize_path(file_path),
            label: normalize_path(file_path)
                .split('/')
                .next_back()
                .unwrap_or(file_path)
                .to_string(),
            kind: NodeKind::File,
            file_path: normalize_path(file_path),
            line: None,
            is_public: true,
                parent_id: None,
        });

        let source_bytes = source.as_bytes();
        self.traverse_tree(&tree, source_bytes, file_path, &mut graph)?;

        graph.meta.node_count = graph.nodes.len();
        graph.meta.edge_count = graph.edges.len();

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codeviz_core::ir::{EdgeKind, NodeKind};
    use codeviz_core::parser::LanguageParser;

    #[test]
    fn test_parse_go_snippet() {
        let snippet = r#"
package main

import (
    "fmt"
    "myapp/utils"
)

type Runner interface { Run() }
type Dog struct { Animal }
func (d *Dog) Run() {}
func main() {}
"#;
        let parser = GoParser::new();
        let graph = parser.parse(snippet, "test.go").unwrap();

        // 2 Imports edges
        let imports_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports_edges.len(), 2, "Expected 2 Imports edges");
        assert!(imports_edges.iter().any(|e| e.to_id == "fmt"));
        assert!(imports_edges.iter().any(|e| e.to_id == "myapp/utils"));

        // 1 Interface node
        let interface_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Interface)
            .collect();
        assert_eq!(interface_nodes.len(), 1, "Expected 1 Interface node");
        assert_eq!(interface_nodes[0].label, "Runner");

        // 1 Class node
        let class_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(class_nodes.len(), 1, "Expected 1 Class node");
        assert_eq!(class_nodes[0].label, "Dog");

        // 1 Inherits edge
        let inherits_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits_edges.len(), 1, "Expected 1 Inherits edge");
        assert_eq!(inherits_edges[0].from_id, "test.go::Dog");
        assert_eq!(inherits_edges[0].to_id, "test.go::Animal");

        // 2 Function nodes
        let func_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }))
            .collect();
        assert_eq!(func_nodes.len(), 2, "Expected 2 Function nodes");
        assert!(func_nodes.iter().any(|n| n.label == "Run"));

        let main_func = func_nodes.iter().find(|n| n.label == "main").unwrap();
        assert!(
            main_func.is_public,
            "main must be marked as entry point (is_public: true)"
        );
    }
}
