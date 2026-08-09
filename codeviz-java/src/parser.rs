use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use tree_sitter::{Node as TsNode, Parser, Tree};
use tree_sitter_java::language;

/// A parser for the Java programming language using tree-sitter.
pub struct JavaParser;

impl JavaParser {
    /// Creates a new `JavaParser`.
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
        let mut stack = vec![tree.root_node()];

        while let Some(node) = stack.pop() {
            match node.kind() {
                "import_declaration" => {
                    let mut _is_asterisk = false;
                    let mut c = node.walk();
                    for child in node.children(&mut c) {
                        if child.kind() == "asterisk" {
                            _is_asterisk = true;
                        }
                    }

                    let mut path = String::new();
                    let mut c2 = node.walk();
                    for child in node.children(&mut c2) {
                        if child.kind() == "scoped_identifier" || child.kind() == "identifier" {
                            if let Ok(text) = child.utf8_text(source_bytes) {
                                path = text.to_string();
                            }
                        }
                    }
                    if !path.is_empty() {
                        graph.edges.push(Edge {
                            from_id: file_path.to_string(),
                            to_id: path,
                            kind: EdgeKind::Imports,
                        });
                    }
                }
                "class_declaration" => {
                    self.extract_class(node, source_bytes, file_path, graph)?;
                }
                "interface_declaration" => {
                    self.extract_interface(node, source_bytes, file_path, graph)?;
                }
                "method_declaration" => {
                    self.extract_method(node, source_bytes, file_path, graph)?;
                }
                _ => {}
            }

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                stack.push(child);
            }
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
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", file_path, name);

                let mut modifiers = Vec::new();
                if let Some(mods_node) = node.child_by_field_name("modifiers") {
                    let mut c = mods_node.walk();
                    for child in mods_node.children(&mut c) {
                        if let Ok(mod_text) = child.utf8_text(source_bytes) {
                            if mod_text.starts_with('@') {
                                modifiers.push(format!("[{}]", mod_text));
                            }
                        }
                    }
                }

                let label = if modifiers.is_empty() {
                    name.to_string()
                } else {
                    format!("{} {}", name, modifiers.join(" "))
                };

                graph.nodes.push(Node {
                    id: id.clone(),
                    label,
                    kind: NodeKind::Class,
                    file_path: file_path.to_string(),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public: true,
                });

                if let Some(super_node) = node.child_by_field_name("superclass") {
                    let mut c = super_node.walk();
                    for child in super_node.children(&mut c) {
                        if child.kind() == "type_identifier" {
                            if let Ok(super_name) = child.utf8_text(source_bytes) {
                                graph.edges.push(Edge {
                                    from_id: id.clone(),
                                    to_id: format!("{}::{}", file_path, super_name),
                                    kind: EdgeKind::Inherits,
                                });
                            }
                        }
                    }
                }

                if let Some(interfaces_node) = node.child_by_field_name("interfaces") {
                    let mut c = interfaces_node.walk();
                    for child in interfaces_node.children(&mut c) {
                        if child.kind() == "type_list" {
                            let mut lc = child.walk();
                            for tchild in child.children(&mut lc) {
                                if tchild.kind() == "type_identifier" {
                                    if let Ok(iface_name) = tchild.utf8_text(source_bytes) {
                                        graph.edges.push(Edge {
                                            from_id: id.clone(),
                                            to_id: format!("{}::{}", file_path, iface_name),
                                            kind: EdgeKind::Implements,
                                        });
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

    fn extract_interface(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", file_path, name);

                let mut modifiers = Vec::new();
                if let Some(mods_node) = node.child_by_field_name("modifiers") {
                    let mut c = mods_node.walk();
                    for child in mods_node.children(&mut c) {
                        if let Ok(mod_text) = child.utf8_text(source_bytes) {
                            if mod_text.starts_with('@') {
                                modifiers.push(format!("[{}]", mod_text));
                            }
                        }
                    }
                }

                let label = if modifiers.is_empty() {
                    name.to_string()
                } else {
                    format!("{} {}", name, modifiers.join(" "))
                };

                graph.nodes.push(Node {
                    id: id.clone(),
                    label,
                    kind: NodeKind::Interface,
                    file_path: file_path.to_string(),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public: true,
                });

                if let Some(extends_node) = node.child_by_field_name("interfaces") {
                    let mut c = extends_node.walk();
                    for child in extends_node.children(&mut c) {
                        if child.kind() == "type_list" {
                            let mut lc = child.walk();
                            for tchild in child.children(&mut lc) {
                                if tchild.kind() == "type_identifier" {
                                    if let Ok(iface_name) = tchild.utf8_text(source_bytes) {
                                        graph.edges.push(Edge {
                                            from_id: id.clone(),
                                            to_id: format!("{}::{}", file_path, iface_name),
                                            kind: EdgeKind::Inherits, // For interfaces, extends is Inherits
                                        });
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

    fn extract_method(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        // Skip interface method declarations that might just be named differently, wait, Java interfaces have methods that are often just method_declaration in tree-sitter.
        // Wait, looking at the debug output, `run` appears twice.
        // One is from `interface Runnable { void run(); }`
        // The other is from `class Dog ... { public void run() {} }`

        // The acceptance criteria expects EXACTLY 2 Function nodes.
        // Given `run` and `main`, this means we should NOT capture interface method declarations as Function nodes, or we should deduplicate, or something else.
        // Actually, if we look at `class Dog { public void run() {} public static void main(String[] args) {} }`
        // The two functions expected are `run` and `main` in Dog class? Or just 2 overall?
        // Wait, `run` in `Runnable` and `run` in `Dog`. That's 3 if we include `main`. But the spec says: "2 Function nodes (`run`, `main`)".
        // This implies interface method declarations should be skipped, OR we just skip them if they have no body, etc.
        // In Java, an interface method often has no body (it's a `method_declaration` without a `block`, or maybe just `method_declaration` ending in `;`).
        // Actually, we can check if it has a body. `node.child_by_field_name("body")` exists for methods with bodies.

        // Wait, what if we just check if it has a `block` child? Or `body` field?
        if node.child_by_field_name("body").is_none() {
            return Ok(());
        }

        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", file_path, name);

                let mut modifiers = Vec::new();
                let mut is_public = false;
                if let Some(mods_node) = node.child_by_field_name("modifiers") {
                    let mut c = mods_node.walk();
                    for child in mods_node.children(&mut c) {
                        if let Ok(mod_text) = child.utf8_text(source_bytes) {
                            if mod_text.starts_with('@') {
                                modifiers.push(format!("[{}]", mod_text));
                            }
                            if mod_text == "public" {
                                is_public = true;
                            }
                        }
                    }
                }
                if name == "main" {
                    is_public = true; // Mark entry point
                }

                let label = if modifiers.is_empty() {
                    name.to_string()
                } else {
                    format!("{} {}", name, modifiers.join(" "))
                };

                graph.nodes.push(Node {
                    id,
                    label,
                    kind: NodeKind::Function { is_async: false },
                    file_path: file_path.to_string(),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public,
                });
            }
        }
        Ok(())
    }
}

impl Default for JavaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for JavaParser {
    fn language_name(&self) -> &str {
        "java"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["java"]
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();
        if parser.set_language(&language()).is_err() {
            return Err(ParseError {
                message: "Failed to set Java language for tree-sitter".to_string(),
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
            id: file_path.to_string(),
            label: file_path
                .split('/')
                .next_back()
                .unwrap_or(file_path)
                .to_string(),
            kind: NodeKind::File,
            file_path: file_path.to_string(),
            line: None,
            is_public: true,
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
    fn test_parse_java_snippet() {
        let snippet = r#"
import java.util.List;
import java.io.*;

interface Runnable { void run(); }
class Animal {}
class Dog extends Animal implements Runnable {
    @Override
    public void run() {}
    public static void main(String[] args) {}
}
"#;
        let parser = JavaParser::new();
        let graph = parser.parse(snippet, "test.java").unwrap();

        // 2 Imports edges
        let imports_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports_edges.len(), 2, "Expected 2 Imports edges");
        assert!(imports_edges.iter().any(|e| e.to_id == "java.util.List"));
        assert!(imports_edges.iter().any(|e| e.to_id == "java.io"));

        // 1 Interface node
        let interface_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Interface)
            .collect();
        assert_eq!(interface_nodes.len(), 1, "Expected 1 Interface node");
        assert_eq!(interface_nodes[0].label, "Runnable");

        // 2 Class nodes
        let class_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(class_nodes.len(), 2, "Expected 2 Class nodes");
        assert!(class_nodes.iter().any(|n| n.label == "Animal"));
        assert!(class_nodes.iter().any(|n| n.label == "Dog"));

        // 1 Inherits edge
        let inherits_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits_edges.len(), 1, "Expected 1 Inherits edge");
        assert_eq!(inherits_edges[0].from_id, "test.java::Dog");
        assert_eq!(inherits_edges[0].to_id, "test.java::Animal");

        // 1 Implements edge
        let implements_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Implements)
            .collect();
        assert_eq!(implements_edges.len(), 1, "Expected 1 Implements edge");
        assert_eq!(implements_edges[0].from_id, "test.java::Dog");
        assert_eq!(implements_edges[0].to_id, "test.java::Runnable");

        // 2 Function nodes
        let func_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }))
            .collect();
        assert_eq!(func_nodes.len(), 2, "Expected 2 Function nodes");
        assert!(func_nodes.iter().any(|n| n.label.contains("run")));

        let main_func = func_nodes.iter().find(|n| n.label == "main").unwrap();
        assert!(
            main_func.is_public,
            "main must be marked as entry point (is_public: true)"
        );
    }
}
