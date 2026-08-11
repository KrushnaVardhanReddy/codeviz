use codeviz_core::path_utils::normalize_path;
use codeviz_core::{
    CodeGraph, Edge, EdgeKind, GraphMeta, LanguageParser, Node, NodeKind, ParseError,
};
use tree_sitter::{Node as TSNode, Parser};

/// Kotlin parser for extracting code graph information.
pub struct KotlinParser;

impl KotlinParser {
    /// Creates a new KotlinParser instance.
    pub fn new() -> Self {
        Self {}
    }

    fn traverse<'a>(&self, node: TSNode<'a>, source: &str, graph: &mut CodeGraph, file_path: &str) {
        let kind = node.kind();

        match kind {
            "import_header" => self.handle_import(node, source, graph, file_path),
            "class_declaration" => self.handle_class_declaration(node, source, graph, file_path),
            "object_declaration" => self.handle_object_declaration(node, source, graph, file_path),
            "function_declaration" => {
                self.handle_function_declaration(node, source, graph, file_path)
            }
            "companion_object" => {
                let line = node.start_position().row as u32 + 1;
                graph.nodes.push(Node {
                    id: "companion_object".to_string(),
                    label: "Companion[object]".to_string(),
                    kind: NodeKind::Class,
                    file_path: normalize_path(file_path),
                    line: Some(line),
                    is_public: true,
                });
                let mut c = node.walk();
                for child in node.children(&mut c) {
                    self.traverse(child, source, graph, file_path);
                }
            }
            "infix_expression" => {
                let mut c = node.walk();
                for child in node.children(&mut c) {
                    self.traverse(child, source, graph, file_path);
                }
            }
            "object_literal" | "lambda_literal" | "ERROR" => {
                // Skip anonymous objects, lambdas, and ERROR nodes (which naturally skips abstract interface methods)
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.traverse(child, source, graph, file_path);
                }
            }
        }
    }

    fn handle_import<'a>(
        &self,
        node: TSNode<'a>,
        source: &str,
        graph: &mut CodeGraph,
        file_path: &str,
    ) {
        let mut cursor = node.walk();
        let mut target = String::new();

        for child in node.children(&mut cursor) {
            if child.kind() == "identifier" {
                target = child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            } else if child.kind() == "wildcard_import" {
                // Ignore wildcard since we already have the identifier
            }
        }

        if !target.is_empty() {
            graph.edges.push(Edge {
                from_id: file_path.to_string(),
                to_id: target,
                kind: EdgeKind::Imports,
            });
        }
    }

    fn handle_class_declaration<'a>(
        &self,
        node: TSNode<'a>,
        source: &str,
        graph: &mut CodeGraph,
        file_path: &str,
    ) {
        let mut cursor = node.walk();
        let mut is_interface = false;
        let mut is_data = false;
        let mut class_name = String::new();
        let line = node.start_position().row as u32 + 1;

        for child in node.children(&mut cursor) {
            if child.kind() == "interface" {
                is_interface = true;
            } else if child.kind() == "type_identifier" {
                class_name = child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            } else if child.kind() == "modifiers" {
                let mut mod_cursor = child.walk();
                for mod_child in child.children(&mut mod_cursor) {
                    if mod_child.kind() == "class_modifier" {
                        let mut mod_mod_cursor = mod_child.walk();
                        for mm_child in mod_child.children(&mut mod_mod_cursor) {
                            if mm_child.kind() == "data" {
                                is_data = true;
                            }
                        }
                    }
                }
            }
        }

        if class_name.is_empty() {
            return;
        }

        let label = if is_data {
            format!("{}[data]", class_name)
        } else {
            class_name.clone()
        };
        let kind = if is_interface {
            NodeKind::Interface
        } else {
            NodeKind::Class
        };

        graph.nodes.push(Node {
            id: class_name.clone(),
            label,
            kind,
            file_path: normalize_path(file_path),
            line: Some(line),
            is_public: true,
        });

        // Handle delegation_specifier (inheritance/implements)
        let mut ds_cursor = node.walk();
        for child in node.children(&mut ds_cursor) {
            if child.kind() == "delegation_specifier" {
                let mut inner_cursor = child.walk();
                for inner_child in child.children(&mut inner_cursor) {
                    if inner_child.kind() == "constructor_invocation" {
                        // Inherits
                        let mut tc_cursor = inner_child.walk();
                        for tc in inner_child.children(&mut tc_cursor) {
                            if tc.kind() == "user_type" {
                                let target =
                                    tc.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                                graph.edges.push(Edge {
                                    from_id: class_name.clone(),
                                    to_id: target,
                                    kind: EdgeKind::Inherits,
                                });
                            }
                        }
                    } else if inner_child.kind() == "user_type" {
                        // Implements for class, Inherits for interface
                        let target = inner_child
                            .utf8_text(source.as_bytes())
                            .unwrap_or("")
                            .to_string();
                        graph.edges.push(Edge {
                            from_id: class_name.clone(),
                            to_id: target,
                            kind: if is_interface {
                                EdgeKind::Inherits
                            } else {
                                EdgeKind::Implements
                            },
                        });
                    }
                }
            }
        }

        // Process children
        let mut b_cursor = node.walk();
        for child in node.children(&mut b_cursor) {
            if child.kind() == "class_body" {
                let mut body_cursor = child.walk();
                for body_child in child.children(&mut body_cursor) {
                    self.traverse(body_child, source, graph, file_path);
                }
            }
        }
    }

    fn handle_object_declaration<'a>(
        &self,
        node: TSNode<'a>,
        source: &str,
        graph: &mut CodeGraph,
        file_path: &str,
    ) {
        let mut cursor = node.walk();
        let mut object_name = String::new();
        let line = node.start_position().row as u32 + 1;

        for child in node.children(&mut cursor) {
            if child.kind() == "type_identifier" {
                object_name = child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            }
        }

        if !object_name.is_empty() {
            graph.nodes.push(Node {
                id: object_name.clone(),
                label: format!("{}[object]", object_name),
                kind: NodeKind::Class,
                file_path: normalize_path(file_path),
                line: Some(line),
                is_public: true,
            });
        }

        let mut b_cursor = node.walk();
        for child in node.children(&mut b_cursor) {
            if child.kind() == "class_body" {
                let mut body_cursor = child.walk();
                for body_child in child.children(&mut body_cursor) {
                    self.traverse(body_child, source, graph, file_path);
                }
            }
        }
    }

    fn handle_function_declaration<'a>(
        &self,
        node: TSNode<'a>,
        source: &str,
        graph: &mut CodeGraph,
        file_path: &str,
    ) {
        let mut cursor = node.walk();
        let mut func_name = String::new();
        let mut is_async = false;
        let line = node.start_position().row as u32 + 1;

        for child in node.children(&mut cursor) {
            if child.kind() == "simple_identifier" {
                func_name = child.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            } else if child.kind() == "modifiers" {
                let mut mod_cursor = child.walk();
                for mod_child in child.children(&mut mod_cursor) {
                    if mod_child.kind() == "function_modifier" {
                        let mut mm_cursor = mod_child.walk();
                        for mm_child in mod_child.children(&mut mm_cursor) {
                            let kind = mm_child.kind();
                            if kind == "suspend" {
                                is_async = true;
                            }
                        }
                    }
                }
            }
        }

        if func_name.is_empty() {
            return;
        }

        graph.nodes.push(Node {
            id: func_name.clone(),
            label: func_name.clone(),
            kind: NodeKind::Function { is_async },
            file_path: normalize_path(file_path),
            line: Some(line),
            is_public: true,
        });
    }
}

impl Default for KotlinParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for KotlinParser {
    fn language_name(&self) -> &str {
        "Kotlin"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["kt", "kts"]
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_kotlin::language())
            .map_err(|e| ParseError {
                message: format!("Failed to set Kotlin language: {}", e),
                file_path: file_path.to_string(),
                line: None,
            })?;

        let tree = parser.parse(source, None).ok_or_else(|| ParseError {
            message: "Failed to parse Kotlin source".to_string(),
            file_path: file_path.to_string(),
            line: None,
        })?;

        let mut graph = CodeGraph::new(GraphMeta {
            language: "kotlin".to_string(),
            source_root: file_path.to_string(),
            generated_at: "".to_string(),
            node_count: 0,
            edge_count: 0,
        });

        self.traverse(tree.root_node(), source, &mut graph, file_path);

        graph.meta.node_count = graph.nodes.len();
        graph.meta.edge_count = graph.edges.len();

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kotlin_parser() {
        let parser = KotlinParser::new();
        let source_code = r#"
import java.util.List

interface Runnable { fun run() }
open class Animal
class Dog : Animal(), Runnable {
    override fun run() {}
    suspend fun fetch() {}
}
"#;
        let graph = parser.parse(source_code, "test.kt").unwrap();

        // 1 Imports edge
        let imports_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports_edges.len(), 1);
        assert_eq!(imports_edges[0].to_id, "java.util.List");

        // 1 Interface node (Runnable)
        let interfaces: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Interface)
            .collect();
        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].id, "Runnable");

        // 2 Class nodes (Animal, Dog)
        let classes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(classes.len(), 2);
        assert!(classes.iter().any(|n| n.id == "Animal"));
        assert!(classes.iter().any(|n| n.id == "Dog"));

        // 1 Inherits edge (Dog -> Animal)
        let inherits: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits.len(), 1);
        assert_eq!(inherits[0].from_id, "Dog");
        assert_eq!(inherits[0].to_id, "Animal");

        // 1 Implements edge (Dog -> Runnable)
        let implements: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Implements)
            .collect();
        assert_eq!(implements.len(), 1);
        assert_eq!(implements[0].from_id, "Dog");
        assert_eq!(implements[0].to_id, "Runnable");

        // 2 Function nodes (run, fetch)
        let functions: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }))
            .collect();
        assert_eq!(functions.len(), 2);

        let fetch_fn = functions.iter().find(|n| n.id == "fetch").unwrap();
        assert!(matches!(
            fetch_fn.kind,
            NodeKind::Function { is_async: true }
        ));

        let run_fn = functions.iter().find(|n| n.id == "run").unwrap();
        assert!(matches!(
            run_fn.kind,
            NodeKind::Function { is_async: false }
        ));
    }
}
