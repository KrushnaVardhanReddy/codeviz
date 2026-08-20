use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use codeviz_core::path_utils::normalize_path;
use std::collections::HashMap;
use tree_sitter::{Node as TsNode, Parser, Tree};
use tree_sitter_python::language;

/// A parser for the Python programming language using tree-sitter.
pub struct PythonParser;

impl PythonParser {
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
        let mut scope_map: HashMap<String, String> = HashMap::new();
        let mut nodes_to_visit = vec![(tree.root_node(), None)];

        while let Some((node, parent_id)) = nodes_to_visit.pop() {
            let mut next_parent = parent_id.clone();

            match node.kind() {
                "class_definition" => {
                    if let Some(id) =
                        self.extract_class(node, source_bytes, file_path, graph, &parent_id)?
                    {
                        next_parent = Some(id);
                    }
                }
                "function_definition" => {
                    let is_async = node.child(0).is_some_and(|c| c.kind() == "async");
                    if let Some(id) = self.extract_function(
                        node,
                        source_bytes,
                        file_path,
                        graph,
                        is_async,
                        &parent_id,
                    )? {
                        next_parent = Some(id);
                    }
                }
                "import_statement" => {
                    self.extract_import(node, source_bytes, file_path, graph)?
                }
                "import_from_statement" => {
                    self.extract_import_from(node, source_bytes, file_path, graph, &mut scope_map)?
                }
                "decorated_definition" => {
                    if let Some((target_node, target_id)) =
                        self.extract_decorated(node, source_bytes, file_path, graph, &parent_id)?
                    {
                        let mut cursor = target_node.walk();
                        let mut children = vec![];
                        for child in target_node.children(&mut cursor) {
                            children.push((child, Some(target_id.clone())));
                        }
                        children.reverse();
                        nodes_to_visit.extend(children);
                    }
                }
                "call" => {
                    if let Some(ref caller_id) = parent_id {
                        self.extract_call(node, source_bytes, file_path, graph, caller_id, &scope_map)?;
                    }
                }
                _ => {}
            }

            let mut cursor = node.walk();
            let mut children = vec![];
            for child in node.children(&mut cursor) {
                if node.kind() != "decorated_definition" {
                    children.push((child, next_parent.clone()));
                }
            }
            children.reverse();
            nodes_to_visit.extend(children);
        }
        Ok(())
    }

    fn extract_class(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        parent_id: &Option<String>,
    ) -> Result<Option<String>, ParseError> {
        if let Some(parent) = node.parent()
            && parent.kind() == "decorated_definition" {
                return Ok(None);
            }
        self.extract_class_with_decorators(node, source_bytes, file_path, graph, parent_id, &[])
    }

    fn extract_class_with_decorators(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        parent_id: &Option<String>,
        decorators: &[String],
    ) -> Result<Option<String>, ParseError> {
        if let Some(name_node) = node.child_by_field_name("name")
            && let Ok(name) = name_node.utf8_text(source_bytes) {
                let mut label = name.to_string();
                if !decorators.is_empty() {
                    label.push('@');
                    label.push_str(&decorators.join(","));
                }
                let base_id_str = if let Some(p) = parent_id { p.clone() } else { normalize_path(file_path) };
                let id = format!("{}::{}", base_id_str, name);

                graph.nodes.push(Node {
                    parent_id: parent_id.clone().or_else(|| Some(normalize_path(file_path))),
                    id: id.clone(),
                    label,
                    kind: NodeKind::Class,
                    file_path: normalize_path(file_path),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public: true,
                });

                if let Some(p) = parent_id {
                    graph.edges.push(Edge {
                        from_id: p.clone(),
                        to_id: id.clone(),
                        kind: EdgeKind::Contains,
                    });
                } else {
                    graph.edges.push(Edge {
                        from_id: normalize_path(file_path),
                        to_id: id.clone(),
                        kind: EdgeKind::Contains,
                    });
                }

                if let Some(superclasses) = node.child_by_field_name("superclasses") {
                    let mut cursor = superclasses.walk();
                    for child in superclasses.children(&mut cursor) {
                        if (child.kind() == "identifier" || child.kind() == "attribute")
                            && let Ok(base_name) = child.utf8_text(source_bytes)
                        {
                            let to_id = format!("{}::{}", normalize_path(file_path), base_name);
                            graph.edges.push(Edge {
                                from_id: id.clone(),
                                to_id,
                                kind: EdgeKind::Inherits,
                            });
                        }
                    }
                }

                return Ok(Some(id));
            }
        Ok(None)
    }

    fn extract_function(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        is_async: bool,
        parent_id: &Option<String>,
    ) -> Result<Option<String>, ParseError> {
        if let Some(parent) = node.parent()
            && parent.kind() == "decorated_definition" {
                return Ok(None);
            }
        self.extract_function_with_decorators(
            node,
            source_bytes,
            file_path,
            graph,
            is_async,
            parent_id,
            &[],
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn extract_function_with_decorators(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        is_async: bool,
        parent_id: &Option<String>,
        decorators: &[String],
    ) -> Result<Option<String>, ParseError> {
        if let Some(name_node) = node.child_by_field_name("name")
            && let Ok(name) = name_node.utf8_text(source_bytes) {
                let mut label = name.to_string();
                if !decorators.is_empty() {
                    label.push('@');
                    label.push_str(&decorators.join(","));
                }
                
                let base_id_str = if let Some(p) = parent_id { p.clone() } else { normalize_path(file_path) };
                let id = format!("{}::{}", base_id_str, name);

                graph.nodes.push(Node {
                    parent_id: parent_id.clone().or_else(|| Some(normalize_path(file_path))),
                    id: id.clone(),
                    label,
                    kind: NodeKind::Function { is_async },
                    file_path: normalize_path(file_path),
                    line: Some(name_node.start_position().row as u32 + 1),
                    is_public: true,
                });

                if let Some(p) = parent_id {
                    graph.edges.push(Edge {
                        from_id: p.clone(),
                        to_id: id.clone(),
                        kind: EdgeKind::Contains,
                    });
                } else {
                    graph.edges.push(Edge {
                        from_id: normalize_path(file_path),
                        to_id: id.clone(),
                        kind: EdgeKind::Contains,
                    });
                }

                return Ok(Some(id));
            }
        Ok(None)
    }

    fn extract_decorated<'a>(
        &self,
        node: TsNode<'a>,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        parent_id: &Option<String>,
    ) -> Result<Option<(TsNode<'a>, String)>, ParseError> {
        let mut decorators = Vec::new();
        let mut target_node = None;
        let mut is_async = false;

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "decorator" => {
                    let mut id_cursor = child.walk();
                    for id_child in child.children(&mut id_cursor) {
                        if (id_child.kind() == "identifier"
                            || id_child.kind() == "call"
                            || id_child.kind() == "dotted_name")
                            && let Ok(text) = id_child.utf8_text(source_bytes) {
                                decorators.push(text.split('(').next().unwrap_or(text).to_string());
                            }
                    }
                }
                "class_definition" => {
                    target_node = Some(child);
                }
                "function_definition" => {
                    target_node = Some(child);
                    is_async = child.child(0).is_some_and(|c| c.kind() == "async");
                }
                _ => {}
            }
        }

        if let Some(target) = target_node {
            let id = match target.kind() {
                "class_definition" => self.extract_class_with_decorators(
                    target,
                    source_bytes,
                    file_path,
                    graph,
                    parent_id,
                    &decorators,
                )?,
                "function_definition" => self.extract_function_with_decorators(
                    target,
                    source_bytes,
                    file_path,
                    graph,
                    is_async,
                    parent_id,
                    &decorators,
                )?,
                _ => None,
            };
            if let Some(id_str) = id {
                return Ok(Some((target, id_str)));
            }
        }
        Ok(None)
    }

    fn extract_import(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "dotted_name" {
                if let Ok(module_name) = child.utf8_text(source_bytes) {
                    graph.edges.push(Edge {
                        from_id: file_path.to_string(),
                        to_id: module_name.to_string(),
                        kind: EdgeKind::Imports,
                    });
                }
            } else if child.kind() == "aliased_import"
                && let Some(name_node) = child.child_by_field_name("name")
                    && let Ok(module_name) = name_node.utf8_text(source_bytes) {
                        graph.edges.push(Edge {
                            from_id: file_path.to_string(),
                            to_id: module_name.to_string(),
                            kind: EdgeKind::Imports,
                        });
                    }
        }
        Ok(())
    }

    fn extract_import_from(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        scope_map: &mut HashMap<String, String>,
    ) -> Result<(), ParseError> {
        let module_name = node
            .child_by_field_name("module_name")
            .and_then(|n| n.utf8_text(source_bytes).ok())
            .unwrap_or("")
            .to_string();

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "dotted_name"
                || child.kind() == "aliased_import"
                || child.kind() == "identifier"
            {
                let name = child.child_by_field_name("name").unwrap_or(child);
                if let Ok(name_str) = name.utf8_text(source_bytes)
                    && name_str != module_name && name_str != "." {
                        let to_id = if module_name.is_empty() {
                            name_str.to_string()
                        } else {
                            format!("{}.{}", module_name, name_str)
                        };
                        graph.edges.push(Edge {
                            from_id: file_path.to_string(),
                            to_id: to_id.clone(),
                            kind: EdgeKind::Imports,
                        });
                        scope_map.insert(name_str.to_string(), to_id);
                    }
            }
        }
        Ok(())
    }

    fn extract_call(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        _file_path: &str,
        graph: &mut CodeGraph,
        caller_id: &str,
        scope_map: &HashMap<String, String>,
    ) -> Result<(), ParseError> {
        if let Some(func_node) = node.child_by_field_name("function")
            && let Ok(func_name) = func_node.utf8_text(source_bytes) {
                let to_id = if let Some(method_name) = func_name.strip_prefix("self.") {
                    if let Some(idx) = caller_id.rfind("::") {
                        format!("{}::{}", &caller_id[..idx], method_name)
                    } else {
                        func_name.to_string()
                    }
                } else if let Some(resolved) = scope_map.get(func_name) {
                    resolved.clone()
                } else {
                    func_name.to_string()
                };

                graph.edges.push(Edge {
                    from_id: caller_id.to_string(),
                    to_id,
                    kind: EdgeKind::Calls,
                });
            }
        Ok(())
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

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();
        if parser.set_language(&language()).is_err() {
            return Err(ParseError {
                message: "Failed to set Python language for tree-sitter".to_string(),
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

        let mut graph = CodeGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            control_flow: Vec::new(),
            meta: GraphMeta {
                language: "python".to_string(),
                source_root: String::new(),
                generated_at: String::new(),
                node_count: 0,
                edge_count: 0,
            },
        };
        
        let normalized = normalize_path(file_path);
        graph.nodes.push(Node {
            parent_id: None,
            id: normalized.clone(),
            label: normalized.clone(),
            kind: NodeKind::Module,
            file_path: normalized.clone(),
            line: Some(1),
            is_public: true,
        });

        self.traverse_tree(&tree, source.as_bytes(), file_path, &mut graph)?;

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_python_snippet() {
        let parser = PythonParser::new();
        let source = r#"
import os
from collections import defaultdict

class Graph:
    def __init__(self):
        self.nodes = defaultdict(list)

    def add_node(self, node):
        self.nodes[node] = True
        os.path.join("a", "b")

@dataclass
class Edge:
    source: str
    target: str
"#;
        let graph = parser.parse(source, "test.py").unwrap();
        
        assert!(graph.nodes.iter().any(|n| n.label == "Graph" && n.parent_id.as_deref() == Some("test.py")));
        assert!(graph.nodes.iter().any(|n| n.label == "Edge@dataclass"));
        assert!(graph.edges.iter().any(|e| e.kind == EdgeKind::Imports && e.to_id == "os"));
        assert!(graph.edges.iter().any(|e| e.kind == EdgeKind::Imports && e.to_id == "collections.defaultdict"));
        assert!(graph.edges.iter().any(|e| e.kind == EdgeKind::Calls && e.from_id == "test.py::Graph::add_node" && e.to_id == "os.path.join"));
    }
}
