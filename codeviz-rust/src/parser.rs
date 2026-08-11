use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use codeviz_core::path_utils::normalize_path;
use tree_sitter::{Node as TsNode, Parser, Tree};
use tree_sitter_rust::language;

/// A parser for the Rust programming language using tree-sitter.
pub struct RustLangParser;

impl RustLangParser {
    /// Creates a new `RustLangParser`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for RustLangParser {
    fn default() -> Self {
        Self::new()
    }
}

impl RustLangParser {
    fn traverse_tree(
        &self,
        tree: &Tree,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let mut nodes_to_visit = vec![tree.root_node()];

        while let Some(node) = nodes_to_visit.pop() {
            match node.kind() {
                "use_declaration" => {
                    self.extract_use(node, source_bytes, file_path, graph)?;
                }
                "mod_item" => {
                    self.extract_mod(node, source_bytes, file_path, graph)?;
                }
                "extern_crate_declaration" => {
                    self.extract_extern_crate(node, source_bytes, file_path, graph)?;
                }
                "struct_item" | "enum_item" => {
                    self.extract_class(node, source_bytes, file_path, graph)?;
                }
                "trait_item" => {
                    self.extract_trait(node, source_bytes, file_path, graph)?;
                }
                "impl_item" => {
                    self.extract_impl(node, source_bytes, file_path, graph)?;
                }
                "function_item" => {
                    self.extract_function(node, source_bytes, file_path, graph)?;
                }
                _ => {}
            }

            let mut child_cursor = node.walk();
            for child in node.children(&mut child_cursor) {
                if child.kind() != "macro_invocation" && child.kind() != "attribute_item" {
                    nodes_to_visit.push(child);
                }
            }
        }
        Ok(())
    }

    fn get_text<'a>(&self, node: TsNode<'a>, source_bytes: &'a [u8]) -> String {
        node.utf8_text(source_bytes).unwrap_or("").to_string()
    }

    fn is_public(&self, node: TsNode) -> bool {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "visibility_modifier" {
                return true;
            }
        }
        false
    }

    fn sanitize_label(&self, text: &str) -> String {
        let mut s = text.to_string();
        // naive generic/lifetime stripping
        if let Some(idx) = s.find('<') {
            s = s[..idx].to_string();
        }
        if let Some(idx) = s.find(" where") {
            s = s[..idx].to_string();
        }
        s.trim().to_string()
    }

    fn extract_use(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "scoped_identifier"
                || child.kind() == "identifier"
                || child.kind() == "use_list"
            {
                self.extract_use_path(child, source_bytes, file_path, graph, "")?;
            }
        }
        Ok(())
    }

    fn extract_use_path(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        prefix: &str,
    ) -> Result<(), ParseError> {
        if node.kind() == "scoped_identifier" {
            let path_str = self.get_text(node, source_bytes);
            let target = if prefix.is_empty() {
                path_str
            } else {
                format!("{}::{}", prefix, path_str)
            };

            // if we have 'use std::fmt;' we can just link to std::fmt.
            // if we have 'use std::fmt::Display;' we might also link to std::fmt::Display or just std::fmt. The spec says "extract base module" for use std::collections::HashMap -> std::collections. But simple extraction is often enough.
            // Let's create an edge to the whole path or base module
            // We'll just split off the last part if it looks like an item, or just emit the whole thing.
            let parts: Vec<&str> = target.split("::").collect();
            let base_module = if parts.len() > 1
                && parts
                    .last()
                    .unwrap_or(&"")
                    .chars()
                    .next()
                    .unwrap_or('a')
                    .is_uppercase()
            {
                parts[..parts.len() - 1].join("::")
            } else {
                target
            };

            graph.edges.push(Edge {
                from_id: normalize_path(file_path),
                to_id: base_module,
                kind: EdgeKind::Imports,
            });
        } else if node.kind() == "identifier" {
            let path_str = self.get_text(node, source_bytes);
            let target = if prefix.is_empty() {
                path_str
            } else {
                format!("{}::{}", prefix, path_str)
            };
            graph.edges.push(Edge {
                from_id: normalize_path(file_path),
                to_id: target,
                kind: EdgeKind::Imports,
            });
        } else if node.kind() == "use_list" {
            // handle use a::{b, c}
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "scoped_identifier" || child.kind() == "identifier" {
                    self.extract_use_path(child, source_bytes, file_path, graph, prefix)?;
                }
            }
        }
        Ok(())
    }

    fn extract_mod(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = self.get_text(name_node, source_bytes);
            graph.edges.push(Edge {
                from_id: normalize_path(file_path),
                to_id: name,
                kind: EdgeKind::Imports,
            });
        }
        Ok(())
    }

    fn extract_extern_crate(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = self.get_text(name_node, source_bytes);
            graph.edges.push(Edge {
                from_id: normalize_path(file_path),
                to_id: name,
                kind: EdgeKind::Imports,
            });
        }
        Ok(())
    }

    fn extract_class(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = self.get_text(name_node, source_bytes);
            let label = self.sanitize_label(&name);

            graph.nodes.push(Node { parent_id: None, 
                id: format!("{}::{}", normalize_path(file_path), label),
                label: label.clone(),
                kind: NodeKind::Class,
                file_path: normalize_path(file_path),
                line: Some(node.start_position().row as u32 + 1),
                is_public: self.is_public(node),
            });
        }
        Ok(())
    }

    fn extract_trait(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = self.get_text(name_node, source_bytes);
            let label = self.sanitize_label(&name);
            let id = format!("{}::{}", normalize_path(file_path), label);

            graph.nodes.push(Node {
                id: id.clone(),
                label: label.clone(),
                kind: NodeKind::Interface,
                file_path: normalize_path(file_path),
                line: Some(node.start_position().row as u32 + 1),
                is_public: self.is_public(node),
                parent_id: None,
            });

            if let Some(bounds_node) = node.child_by_field_name("bounds") {
                let mut cursor = bounds_node.walk();
                for child in bounds_node.children(&mut cursor) {
                    if child.kind() == "trait_bound"
                        || child.kind() == "type_identifier"
                        || child.kind() == "scoped_type_identifier"
                    {
                        let bound_name = self.get_text(child, source_bytes);
                        let bound_label = self.sanitize_label(&bound_name);
                        // Try to get just the trait name if it's something like `fmt::Display`

                        graph.edges.push(Edge {
                            from_id: id.clone(),
                            to_id: bound_label,
                            kind: EdgeKind::Inherits,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_impl(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let trait_node = node.child_by_field_name("trait");
        let type_node = node.child_by_field_name("type");

        if let Some(t_node) = type_node {
            let t_name = self.get_text(t_node, source_bytes);
            let t_label = self.sanitize_label(&t_name);
            let from_id = format!("{}::{}", file_path, t_label);

            if let Some(tr_node) = trait_node {
                let tr_name = self.get_text(tr_node, source_bytes);
                let tr_label = self.sanitize_label(&tr_name);

                graph.edges.push(Edge {
                    from_id,
                    to_id: tr_label, // Should we resolve this? Yes, typically we just store the name
                    kind: EdgeKind::Implements,
                });
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
            let name = self.get_text(name_node, source_bytes);
            let label = self.sanitize_label(&name);

            let mut is_async = false;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "function_modifiers" {
                    let mut mod_cursor = child.walk();
                    for mod_child in child.children(&mut mod_cursor) {
                        if mod_child.kind() == "async" {
                            is_async = true;
                            break;
                        }
                    }
                }
            }

            graph.nodes.push(Node { parent_id: None, 
                id: format!("{}::{}", normalize_path(file_path), label),
                label: label.clone(),
                kind: NodeKind::Function { is_async },
                file_path: normalize_path(file_path),
                line: Some(node.start_position().row as u32 + 1),
                is_public: self.is_public(node),
            });
        }
        Ok(())
    }
}

impl LanguageParser for RustLangParser {
    fn language_name(&self) -> &str {
        "rust"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["rs"]
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();
        if parser.set_language(&language()).is_err() {
            return Err(ParseError {
                message: "Failed to set Rust language for tree-sitter".to_string(),
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

        if tree.root_node().has_error() {
            return Err(ParseError {
                message: "Syntax error".to_string(),
                file_path: file_path.to_string(),
                line: Some(tree.root_node().start_position().row as u32 + 1),
            });
        }

        let mut graph = CodeGraph::new(GraphMeta {
            language: self.language_name().to_string(),
            source_root: "".to_string(), // Caller should modify this later if needed
            generated_at: "".to_string(),
            node_count: 0,
            edge_count: 0,
        });

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

    #[test]
    fn test_parse_rust_snippet() {
        let snippet = r#"
use std::fmt;

trait Greet: fmt::Display {}
struct Dog;
impl Greet for Dog {}
pub async fn bark() {}
"#;
        let parser = RustLangParser::new();
        let graph = parser.parse(snippet, "test.rs").unwrap();

        // 1 `Imports` edge (`std::fmt`)
        let imports: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports.len(), 1, "Expected 1 Imports edge");
        assert_eq!(imports[0].to_id, "std::fmt", "Imports should be std::fmt");

        // 1 `Interface` node (`Greet`)
        let interfaces: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Interface)
            .collect();
        assert_eq!(interfaces.len(), 1, "Expected 1 Interface node");
        assert_eq!(interfaces[0].label, "Greet", "Interface should be Greet");

        // 1 `Inherits` edge (`Greet` -> `fmt::Display`)
        let inherits: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits.len(), 1, "Expected 1 Inherits edge");
        assert_eq!(inherits[0].from_id, "test.rs::Greet");
        assert_eq!(inherits[0].to_id, "fmt::Display");

        // 1 `Class` node (`Dog`)
        let classes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(classes.len(), 1, "Expected 1 Class node");
        assert_eq!(classes[0].label, "Dog");

        // 1 `Implements` edge (`Dog` -> `Greet`)
        let implements: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Implements)
            .collect();
        assert_eq!(implements.len(), 1, "Expected 1 Implements edge");
        assert_eq!(implements[0].from_id, "test.rs::Dog");
        assert_eq!(implements[0].to_id, "Greet");

        // 1 async `Function` node (`bark`) with `is_public: true`
        let funcs: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }))
            .collect();
        assert_eq!(funcs.len(), 1, "Expected 1 Function node");
        assert_eq!(funcs[0].label, "bark");
        assert_eq!(funcs[0].is_public, true);
        if let NodeKind::Function { is_async } = funcs[0].kind {
            assert!(is_async, "Function should be async");
        } else {
            panic!("Expected Function node");
        }
    }
}
