use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use codeviz_core::path_utils::normalize_path;
use tree_sitter::{Node as TsNode, Parser, Tree};
use tree_sitter_python::language;

/// A parser for the Python programming language using tree-sitter.
pub struct PythonParser;

impl PythonParser {
    /// Creates a new `PythonParser`.
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
        let mut nodes_to_visit = vec![tree.root_node()];

        while let Some(node) = nodes_to_visit.pop() {
            match node.kind() {
                "class_definition" => {
                    self.extract_class(node, source_bytes, file_path, graph)?;
                }
                "function_definition" => {
                    let is_async = node.child(0).is_some_and(|c| c.kind() == "async");
                    self.extract_function(node, source_bytes, file_path, graph, is_async)?;
                }
                "import_statement" => {
                    self.extract_import(node, source_bytes, file_path, graph)?;
                }
                "import_from_statement" => {
                    self.extract_import_from(node, source_bytes, file_path, graph)?;
                }
                "decorated_definition" => {
                    let target = self.extract_decorated(node, source_bytes, file_path, graph)?;
                    if let Some(target) = target {
                        // Push the target node so we traverse its children (e.g. methods in a decorated class)

                        let mut child_cursor = target.walk();
                        for child in target.children(&mut child_cursor) {
                            nodes_to_visit.push(child);
                        }
                    }
                }
                _ => {}
            }

            let mut child_cursor = node.walk();
            for child in node.children(&mut child_cursor) {
                // If it's a decorated definition, we handle its children specially
                if node.kind() != "decorated_definition"
                    && node.kind() != "class_definition"
                    && node.kind() != "function_definition"
                {
                    nodes_to_visit.push(child);
                } else if node.kind() == "class_definition" || node.kind() == "function_definition"
                {
                    // For regular classes and functions, we want to traverse their children (like block)
                    // to find inner classes/functions/methods.
                    // The children of a class_definition include name, block. We want to search block.
                    nodes_to_visit.push(child);
                }
            }
        }
        Ok(())
    }

    fn extract_decorated<'a>(
        &self,
        node: TsNode<'a>,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<Option<TsNode<'a>>, ParseError> {
        let mut decorators = Vec::new();
        let mut target_node = None;
        let mut is_async = false;

        let mut child_cursor = node.walk();
        for child in node.children(&mut child_cursor) {
            match child.kind() {
                "decorator" => {
                    // Extract decorator name
                    let mut id_cursor = child.walk();
                    for id_child in child.children(&mut id_cursor) {
                        if (id_child.kind() == "identifier"
                            || id_child.kind() == "call"
                            || id_child.kind() == "dotted_name")
                            && let Ok(text) = id_child.utf8_text(source_bytes)
                        {
                            let text = text.split('(').next().unwrap_or(text);
                            decorators.push(text.to_string());
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
            match target.kind() {
                "class_definition" => {
                    self.extract_class_with_decorators(
                        target,
                        source_bytes,
                        file_path,
                        graph,
                        &decorators,
                    )?;
                }
                "function_definition" => {
                    self.extract_function_with_decorators(
                        target,
                        source_bytes,
                        file_path,
                        graph,
                        is_async,
                        &decorators,
                    )?;
                }
                _ => {}
            }
            return Ok(Some(target));
        }

        Ok(None)
    }

    fn extract_class(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        // Check if parent is a decorated_definition. If so, skip because it's handled by extract_decorated
        if let Some(parent) = node.parent()
            && parent.kind() == "decorated_definition"
        {
            return Ok(());
        }
        self.extract_class_with_decorators(node, source_bytes, file_path, graph, &[])
    }

    fn extract_class_with_decorators(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        decorators: &[String],
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name")
            && let Ok(name) = name_node.utf8_text(source_bytes)
        {
            let mut label = name.to_string();
            if !decorators.is_empty() {
                label.push('@');
                label.push_str(&decorators.join(","));
            }

            let id = format!("{}::{}", normalize_path(file_path), name);
            graph.nodes.push(Node { parent_id: None, 
                id: id.clone(),
                label,
                kind: NodeKind::Class,
                file_path: normalize_path(file_path),
                line: Some(name_node.start_position().row as u32 + 1),
                is_public: true, // simplified for python
            });

            if let Some(superclasses) = node.child_by_field_name("superclasses") {
                let mut cursor = superclasses.walk();
                for child in superclasses.children(&mut cursor) {
                    if (child.kind() == "identifier" || child.kind() == "attribute")
                        && let Ok(base_name) = child.utf8_text(source_bytes)
                    {
                        let to_id = format!("{}::{}", normalize_path(file_path), base_name); // Assuming intra-file or external
                        graph.edges.push(Edge {
                            from_id: id.clone(),
                            to_id,
                            kind: EdgeKind::Inherits,
                        });
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
        is_async: bool,
    ) -> Result<(), ParseError> {
        if let Some(parent) = node.parent()
            && parent.kind() == "decorated_definition"
        {
            return Ok(());
        }
        self.extract_function_with_decorators(node, source_bytes, file_path, graph, is_async, &[])
    }

    fn extract_function_with_decorators(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
        is_async: bool,
        decorators: &[String],
    ) -> Result<(), ParseError> {
        if let Some(name_node) = node.child_by_field_name("name")
            && let Ok(name) = name_node.utf8_text(source_bytes)
        {
            let mut label = name.to_string();
            if !decorators.is_empty() {
                label.push('@');
                label.push_str(&decorators.join(","));
            }

            let id = format!("{}::{}", normalize_path(file_path), name);
            graph.nodes.push(Node { parent_id: None, 
                id,
                label,
                kind: NodeKind::Function { is_async },
                file_path: normalize_path(file_path),
                line: Some(name_node.start_position().row as u32 + 1),
                is_public: true, // simplified
            });
        }
        Ok(())
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
                && let Ok(module_name) = name_node.utf8_text(source_bytes)
            {
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
    ) -> Result<(), ParseError> {
        let module_name = if let Some(module_name_node) = node.child_by_field_name("module_name") {
            if let Ok(m) = module_name_node.utf8_text(source_bytes) {
                m.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        let mut found_imports = false;

        let mut cursor = node.walk();
        let module_name_id = node.child_by_field_name("module_name").map(|n| n.id());

        for child in node.children(&mut cursor) {
            if child.kind() == "dotted_name" {
                if Some(child.id()) != module_name_id
                    && let Ok(name) = child.utf8_text(source_bytes)
                {
                    let to_id = if module_name.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}.{}", module_name, name)
                    };
                    graph.edges.push(Edge {
                        from_id: file_path.to_string(),
                        to_id,
                        kind: EdgeKind::Imports,
                    });
                    found_imports = true;
                }
            } else if child.kind() == "aliased_import"
                && let Some(name_node) = child.child_by_field_name("name")
                && let Ok(name) = name_node.utf8_text(source_bytes)
            {
                let to_id = if module_name.is_empty() {
                    name.to_string()
                } else {
                    format!("{}.{}", module_name, name)
                };
                graph.edges.push(Edge {
                    from_id: file_path.to_string(),
                    to_id,
                    kind: EdgeKind::Imports,
                });
                found_imports = true;
            }
        }

        if !found_imports {
            for child in node.children(&mut cursor) {
                if child.kind() == "dotted_name" && Some(child.id()) == module_name_id {
                    continue;
                }

                if (child.kind() == "dotted_name" || child.kind() == "identifier")
                    && let Ok(name) = child.utf8_text(source_bytes)
                    && name != module_name
                    && name != "."
                {
                    // Avoid matching dot as import name when it is a relative import marker
                    let to_id = if module_name.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}.{}", module_name, name)
                    };
                    graph.edges.push(Edge {
                        from_id: file_path.to_string(),
                        to_id,
                        kind: EdgeKind::Imports,
                    });
                }
            }
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

        let mut cfgs = Vec::new();
        let mut cursor = tree.walk();
        let mut stack = vec![tree.root_node()];
        #[allow(clippy::collapsible_if)]
        while let Some(node) = stack.pop() {
            if node.kind() == "function_definition" || node.kind() == "async_function_definition" {
                if let Some(name_node) = node.child_by_field_name("name") {
                    if let Ok(name) = name_node.utf8_text(source_bytes) {
                        let func_id = format!("{}::{}", file_path, name);
                        if let Ok(cfg) = crate::cfg::build_cfg(node, source_bytes, &func_id) {
                            cfgs.push(cfg);
                        }
                    }
                }
            }

            let mut children: Vec<_> = node.children(&mut cursor).collect();
            children.reverse();
            stack.extend(children);
        }
        graph.control_flow = cfgs;

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
    fn test_parse_python_snippet() {
        let snippet = r#"
import os
from pathlib import Path

class Animal:
    pass

class Dog(Animal):
    def bark(self): pass

async def main():
    d = Dog()
"#;
        let parser = PythonParser::new();
        let graph = parser.parse(snippet, "test.py").unwrap();

        let imports_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports_edges.len(), 2, "Expected 2 Imports edges");

        let has_os_import = imports_edges.iter().any(|e| e.to_id == "os");
        assert!(has_os_import, "Expected import for 'os'");

        let has_path_import = imports_edges.iter().any(|e| e.to_id == "pathlib.Path");
        assert!(has_path_import, "Expected import for 'pathlib.Path'");

        let class_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(class_nodes.len(), 2, "Expected 2 Class nodes");

        let has_animal_class = class_nodes.iter().any(|n| n.label == "Animal");
        assert!(has_animal_class, "Expected class 'Animal'");

        let has_dog_class = class_nodes.iter().any(|n| n.label == "Dog");
        assert!(has_dog_class, "Expected class 'Dog'");

        let inherits_edges: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits_edges.len(), 1, "Expected 1 Inherits edge");
        assert_eq!(inherits_edges[0].from_id, "test.py::Dog");
        assert_eq!(inherits_edges[0].to_id, "test.py::Animal");

        let func_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| match n.kind {
                NodeKind::Function { is_async } => is_async,
                _ => false,
            })
            .collect();
        assert_eq!(func_nodes.len(), 1, "Expected 1 async Function node");
        assert_eq!(func_nodes[0].label, "main");

        let bark_func = graph.nodes.iter().find(|n| n.label == "bark");
        assert!(bark_func.is_some(), "Expected function 'bark' to be found");
    }
}

#[cfg(test)]
mod cfg_tests {
    use super::*;
    use codeviz_core::ir::{CfgBlockKind, CfgEdgeKind};

    #[test]
    fn test_python_cfg_generation() {
        let snippet = r#"
async def process_data(data):
    if data:
        await fetch()
    else:
        while True:
            pass
"#;
        let parser = PythonParser::new();
        let graph = parser.parse(snippet, "test_cfg.py").unwrap();

        assert_eq!(graph.control_flow.len(), 1, "Expected 1 CFG");
        let cfg = &graph.control_flow[0];

        let has_condition = cfg.blocks.iter().any(|b| b.kind == CfgBlockKind::Condition);
        assert!(has_condition, "Expected a condition block");

        let has_await = cfg
            .blocks
            .iter()
            .any(|b| b.kind == CfgBlockKind::AwaitPoint);
        assert!(has_await, "Expected an await point block");

        let has_loop_header = cfg
            .blocks
            .iter()
            .any(|b| b.kind == CfgBlockKind::LoopHeader);
        assert!(has_loop_header, "Expected a loop header block");

        let has_true_branch = cfg
            .cfg_edges
            .iter()
            .any(|e| e.kind == CfgEdgeKind::TrueBranch);
        assert!(has_true_branch, "Expected a true branch edge");

        let has_false_branch = cfg
            .cfg_edges
            .iter()
            .any(|e| e.kind == CfgEdgeKind::FalseBranch);
        assert!(has_false_branch, "Expected a false branch edge");
    }
}
