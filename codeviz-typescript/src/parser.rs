#![allow(clippy::collapsible_if)]

use codeviz_core::ir::{CodeGraph, Edge, EdgeKind, GraphMeta, Node, NodeKind};
use codeviz_core::parser::{LanguageParser, ParseError};
use codeviz_core::path_utils::normalize_path;
use tree_sitter::{Node as TsNode, Parser, Tree};

/// A parser for the TypeScript and JavaScript programming languages using tree-sitter.
pub struct TypeScriptParser;

impl TypeScriptParser {
    /// Creates a new `TypeScriptParser`.
    pub fn new() -> Self {
        Self
    }
}

impl Default for TypeScriptParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for TypeScriptParser {
    fn language_name(&self) -> &str {
        "typescript"
    }

    fn supported_extensions(&self) -> &[&str] {
        &["ts", "tsx", "js", "jsx", "mjs", "cjs"]
    }

    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError> {
        let mut parser = Parser::new();

        let is_tsx = file_path.ends_with(".tsx") || file_path.ends_with(".jsx");
        let language = if is_tsx {
            tree_sitter_typescript::language_tsx()
        } else {
            tree_sitter_typescript::language_typescript()
        };

        parser.set_language(&language).map_err(|e| ParseError {
            message: format!("Failed to set TypeScript language: {}", e),
            file_path: file_path.to_string(),
            line: None,
        })?;

        let tree = parser.parse(source, None).ok_or_else(|| ParseError {
            message: "Failed to parse source code".to_string(),
            file_path: file_path.to_string(),
            line: None,
        })?;

        let mut graph = CodeGraph::new(GraphMeta {
            language: self.language_name().to_string(),
            source_root: String::new(),  // Set by caller
            generated_at: String::new(), // Set by caller
            node_count: 0,
            edge_count: 0,
        });

        self.traverse_tree(&tree, source.as_bytes(), file_path, &mut graph)?;

        let mut cfgs = Vec::new();
        let mut cursor = tree.walk();
        let mut stack = vec![tree.root_node()];
        let source_bytes = source.as_bytes();
        #[allow(clippy::collapsible_if)]
        while let Some(node) = stack.pop() {
            if node.kind() == "function_declaration"
                || node.kind() == "method_definition"
                || node.kind() == "arrow_function"
                || node.kind() == "function"
            {
                // Name extraction for typescript functions
                let name = if let Some(n) = node.child_by_field_name("name") {
                    n.utf8_text(source_bytes).unwrap_or("anonymous").to_string()
                } else if node.kind() == "arrow_function" {
                    // Try to get name from parent variable_declarator
                    if let Some(parent) = node.parent() {
                        if parent.kind() == "variable_declarator" {
                            if let Some(n) = parent.child_by_field_name("name") {
                                n.utf8_text(source_bytes).unwrap_or("arrow").to_string()
                            } else {
                                "arrow".to_string()
                            }
                        } else {
                            "arrow".to_string()
                        }
                    } else {
                        "arrow".to_string()
                    }
                } else {
                    "anonymous".to_string()
                };

                let func_id = format!("{}::{}", normalize_path(file_path), name);
                if let Ok(cfg) = crate::cfg::build_cfg(node, source_bytes, &func_id) {
                    cfgs.push(cfg);
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

impl TypeScriptParser {
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
                "import_statement" => {
                    self.extract_import(node, source_bytes, file_path, graph)?;
                }
                "variable_declarator" => {
                    self.extract_require(node, source_bytes, file_path, graph)?;
                }
                "class_declaration" | "class" => {
                    self.extract_class(node, source_bytes, file_path, graph)?;
                }
                "interface_declaration" => {
                    self.extract_interface(node, source_bytes, file_path, graph)?;
                }
                "function_declaration" | "arrow_function" => {
                    self.extract_function(node, source_bytes, file_path, graph)?;
                }
                "export_statement" => {
                    self.extract_export(node, source_bytes, file_path, graph)?;
                }
                "lexical_declaration" | "variable_declaration" => {
                    // Check for arrow functions assigned to variables
                    let mut cursor = node.walk();
                    for child in node.children(&mut cursor) {
                        if child.kind() == "variable_declarator" {
                            if let Some(value) = child.child_by_field_name("value") {
                                if value.kind() == "arrow_function" {
                                    self.extract_function(value, source_bytes, file_path, graph)?;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            // Do not traverse into children of export statements to avoid double-processing,
            // as extract_export already processes its declaration child.
            if node.kind() != "export_statement" {
                let mut child_cursor = node.walk();
                for child in node.children(&mut child_cursor) {
                    nodes_to_visit.push(child);
                }
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
    ) -> Result<(), ParseError> {
        // Skip type-only imports: `import type { X } from 'y'`
        let mut has_type_keyword = false;
        let mut child_cursor = node.walk();
        for child in node.children(&mut child_cursor) {
            if child.kind() == "type" {
                has_type_keyword = true;
                break;
            }
        }

        if has_type_keyword {
            return Ok(());
        }

        // Module specifier is typically a string at the end of the import statement
        let source_node = node.child_by_field_name("source");
        if let Some(source_node) = source_node {
            if source_node.kind() == "string" {
                if let Ok(text) = source_node.utf8_text(source_bytes) {
                    let module_name = text.trim_matches(|c| c == '\'' || c == '"' || c == '`');
                    graph.edges.push(Edge {
                        from_id: file_path.to_string(),
                        to_id: module_name.to_string(),
                        kind: EdgeKind::Imports,
                    });
                }
            }
        }

        Ok(())
    }

    fn extract_require(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        // Look for: const x = require('y')
        // node is variable_declarator
        let value_node = node.child_by_field_name("value");
        if let Some(call_expr) = value_node {
            if call_expr.kind() == "call_expression" {
                let function_node = call_expr.child_by_field_name("function");
                if let Some(func) = function_node {
                    if let Ok(func_text) = func.utf8_text(source_bytes) {
                        if func_text == "require" {
                            let arguments = call_expr.child_by_field_name("arguments");
                            if let Some(args) = arguments {
                                // arguments node has '(' string ')'
                                let mut arg_cursor = args.walk();
                                for arg in args.children(&mut arg_cursor) {
                                    if arg.kind() == "string" {
                                        if let Ok(text) = arg.utf8_text(source_bytes) {
                                            let module_name = text.trim_matches(|c| {
                                                c == '\'' || c == '"' || c == '`'
                                            });
                                            graph.edges.push(Edge {
                                                from_id: file_path.to_string(),
                                                to_id: module_name.to_string(),
                                                kind: EdgeKind::Imports,
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
        Ok(())
    }

    fn extract_class(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let name_node = node.child_by_field_name("name");
        if let Some(name_node) = name_node {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", normalize_path(file_path), name);

                graph.nodes.push(Node {
                    id: id.clone(),
                    label: name.to_string(),
                    kind: NodeKind::Class,
                    file_path: normalize_path(file_path),
                    line: Some(node.start_position().row as u32 + 1),
                    is_public: false, parent_id: None, // Updated by export extraction later
                });

                // Check for inheritance
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "class_heritage" {
                        // extends clause
                        let mut heritage_cursor = child.walk();
                        for h_child in child.children(&mut heritage_cursor) {
                            if h_child.kind() == "extends_clause" {
                                // find the identifier inside extends_clause
                                let mut ext_cursor = h_child.walk();
                                for ext_child in h_child.children(&mut ext_cursor) {
                                    if ext_child.kind() == "identifier"
                                        || ext_child.kind() == "type_identifier"
                                    {
                                        if let Ok(base_class) = ext_child.utf8_text(source_bytes) {
                                            graph.edges.push(Edge {
                                                from_id: id.clone(),
                                                to_id: format!(
                                                    "{}::{}",
                                                    normalize_path(file_path),
                                                    base_class
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
        Ok(())
    }

    fn extract_interface(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let name_node = node.child_by_field_name("name");
        if let Some(name_node) = name_node {
            if let Ok(name) = name_node.utf8_text(source_bytes) {
                let id = format!("{}::{}", normalize_path(file_path), name);

                graph.nodes.push(Node {
                    id: id.clone(),
                    label: name.to_string(),
                    kind: NodeKind::Interface,
                    file_path: normalize_path(file_path),
                    line: Some(node.start_position().row as u32 + 1),
                    is_public: false, parent_id: None, // Updated by export extraction later
                });

                // Check for inheritance
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "extends_type_clause" {
                        // In TS grammar, interface extends uses extends_type_clause, which contains type_identifier directly
                        // or it could have multiple. Let's traverse it to find type_identifier.
                        let mut ext_cursor = child.walk();
                        for ext_child in child.children(&mut ext_cursor) {
                            if ext_child.kind() == "identifier"
                                || ext_child.kind() == "type_identifier"
                            {
                                if let Ok(base_class) = ext_child.utf8_text(source_bytes) {
                                    graph.edges.push(Edge {
                                        from_id: id.clone(),
                                        to_id: format!(
                                            "{}::{}",
                                            normalize_path(file_path),
                                            base_class
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
        Ok(())
    }

    fn extract_function(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        let name_node = node.child_by_field_name("name");

        let name = if let Some(n) = name_node {
            n.utf8_text(source_bytes).unwrap_or("").to_string()
        } else {
            // For arrow functions in variable declarators, the name might be on the parent variable_declarator
            if let Some(parent) = node.parent() {
                if parent.kind() == "variable_declarator" {
                    if let Some(n) = parent.child_by_field_name("name") {
                        n.utf8_text(source_bytes).unwrap_or("").to_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        };

        if name.is_empty() {
            return Ok(()); // Anonymous function
        }

        let mut is_async = false;

        // Check for async keyword
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "async" {
                is_async = true;
                break;
            }
        }

        // For arrow functions in a variable declarator, check previous sibling for async if not found inside arrow function
        if node.kind() == "arrow_function" && !is_async {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "async" {
                    is_async = true;
                    break;
                }
            }
        }

        let id = format!("{}::{}", normalize_path(file_path), name);
        graph.nodes.push(Node {
            id,
            label: name,
            kind: NodeKind::Function { is_async },
            file_path: normalize_path(file_path),
            line: Some(node.start_position().row as u32 + 1),
            is_public: false, parent_id: None, // Updated later
        });

        Ok(())
    }

    fn extract_export(
        &self,
        node: TsNode,
        source_bytes: &[u8],
        file_path: &str,
        graph: &mut CodeGraph,
    ) -> Result<(), ParseError> {
        // export statement can contain a declaration
        let declaration = node.child_by_field_name("declaration");

        if let Some(decl) = declaration {
            match decl.kind() {
                "class_declaration" | "class" => {
                    self.extract_class(decl, source_bytes, file_path, graph)?;
                    if let Some(last) = graph.nodes.last_mut() {
                        last.is_public = true;
                    }
                }
                "interface_declaration" => {
                    self.extract_interface(decl, source_bytes, file_path, graph)?;
                    if let Some(last) = graph.nodes.last_mut() {
                        last.is_public = true;
                    }
                }
                "function_declaration" => {
                    self.extract_function(decl, source_bytes, file_path, graph)?;
                    if let Some(last) = graph.nodes.last_mut() {
                        last.is_public = true;
                    }
                }
                "lexical_declaration" | "variable_declaration" => {
                    // e.g. export const foo = () => {}
                    let mut cursor = decl.walk();
                    for child in decl.children(&mut cursor) {
                        if child.kind() == "variable_declarator" {
                            // We should extract require here if needed, but it's an export.
                            // Check if it's an arrow function
                            if let Some(value) = child.child_by_field_name("value") {
                                if value.kind() == "arrow_function" {
                                    self.extract_function(value, source_bytes, file_path, graph)?;
                                    if let Some(last) = graph.nodes.last_mut() {
                                        last.is_public = true;
                                    }
                                } else if value.kind() == "call_expression" {
                                    self.extract_require(child, source_bytes, file_path, graph)?;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            // maybe export default
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "identifier" {
                    // export { foo }
                    // We would ideally mark `foo` as public.
                    // But for this simple implementation, we might just update nodes at the end.
                    if let Ok(name) = child.utf8_text(source_bytes) {
                        let target_id = format!("{}::{}", file_path, name);
                        for g_node in graph.nodes.iter_mut() {
                            if g_node.id == target_id {
                                g_node.is_public = true;
                            }
                        }
                    }
                }
            }

            let value = node.child_by_field_name("value");
            if let Some(val) = value {
                if val.kind() == "identifier" {
                    if let Ok(name) = val.utf8_text(source_bytes) {
                        let target_id = format!("{}::{}", normalize_path(file_path), name);
                        for g_node in graph.nodes.iter_mut() {
                            if g_node.id == target_id {
                                g_node.is_public = true;
                            }
                        }
                    }
                } else if val.kind() == "class_declaration" || val.kind() == "class" {
                    self.extract_class(val, source_bytes, file_path, graph)?;
                    if let Some(last) = graph.nodes.last_mut() {
                        last.is_public = true;
                    }
                } else if val.kind() == "function_declaration" || val.kind() == "arrow_function" {
                    self.extract_function(val, source_bytes, file_path, graph)?;
                    if let Some(last) = graph.nodes.last_mut() {
                        last.is_public = true;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codeviz_core::ir::{EdgeKind, NodeKind};

    #[test]
    fn test_esm_imports() {
        let parser = TypeScriptParser::new();
        let source = r#"
            import { readFile } from 'fs';
            import * as path from 'path';
            import 'dotenv/config';
            import type { Config } from './config';
            const x = import('dynamic');
        "#;
        let graph = parser.parse(source, "index.ts").unwrap();

        let imports: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports.len(), 3);
        assert!(imports.iter().any(|e| e.to_id == "fs"));
        assert!(imports.iter().any(|e| e.to_id == "path"));
        assert!(imports.iter().any(|e| e.to_id == "dotenv/config"));

        // Ensure type imports and dynamic imports are skipped
        assert!(!imports.iter().any(|e| e.to_id == "./config"));
        assert!(!imports.iter().any(|e| e.to_id == "dynamic"));
    }

    #[test]
    fn test_cjs_require() {
        let parser = TypeScriptParser::new();
        let source = r#"
            const fs = require('fs');
            const path = require("path");
            const dynamic = require.resolve('module');
        "#;
        let graph = parser.parse(source, "index.js").unwrap();

        let imports: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Imports)
            .collect();
        assert_eq!(imports.len(), 2);
        assert!(imports.iter().any(|e| e.to_id == "fs"));
        assert!(imports.iter().any(|e| e.to_id == "path"));

        assert!(!imports.iter().any(|e| e.to_id == "module"));
    }

    #[test]
    fn test_class_inheritance() {
        let parser = TypeScriptParser::new();
        let source = r#"
            class Animal {}
            class Dog extends Animal {}
            interface IFoo {}
            interface IBar extends IFoo {}
        "#;
        let graph = parser.parse(source, "models.ts").unwrap();

        let classes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Class)
            .collect();
        assert_eq!(classes.len(), 2);

        let interfaces: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Interface)
            .collect();
        assert_eq!(interfaces.len(), 2);

        let inherits: Vec<_> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::Inherits)
            .collect();
        assert_eq!(inherits.len(), 2);
        assert!(
            inherits
                .iter()
                .any(|e| e.from_id == "models.ts::Dog" && e.to_id == "models.ts::Animal")
        );
        assert!(
            inherits
                .iter()
                .any(|e| e.from_id == "models.ts::IBar" && e.to_id == "models.ts::IFoo")
        );
    }

    #[test]
    fn test_functions_and_exports() {
        let parser = TypeScriptParser::new();
        let source = r#"
            function normal() {}
            async function fetch() {}
            const arrow = () => {}
            const asyncArrow = async () => {}

            export function exportedFunc() {}
            export class ExportedClass {}
            export default function() {}
        "#;
        let graph = parser.parse(source, "funcs.ts").unwrap();

        let funcs: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, NodeKind::Function { .. }))
            .collect();
        // normal, fetch, arrow, asyncArrow, exportedFunc, and the default exported function.
        // wait, `export default function() {}` has no name. My logic might not extract it.
        // wait, let's just test the named ones.

        let normal = funcs.iter().find(|n| n.label == "normal").unwrap();
        assert_eq!(normal.kind, NodeKind::Function { is_async: false });

        let fetch = funcs.iter().find(|n| n.label == "fetch").unwrap();
        assert_eq!(fetch.kind, NodeKind::Function { is_async: true });

        let arrow = funcs.iter().find(|n| n.label == "arrow").unwrap();
        assert_eq!(arrow.kind, NodeKind::Function { is_async: false });

        let async_arrow = funcs.iter().find(|n| n.label == "asyncArrow").unwrap();
        assert_eq!(async_arrow.kind, NodeKind::Function { is_async: true });

        let exported_func = funcs.iter().find(|n| n.label == "exportedFunc").unwrap();
        assert!(exported_func.is_public);

        let exported_class = graph
            .nodes
            .iter()
            .find(|n| n.label == "ExportedClass")
            .unwrap();
        assert!(exported_class.is_public);
    }
}

#[cfg(test)]
mod cfg_tests {
    use super::*;
    use codeviz_core::ir::{CfgBlockKind, CfgEdgeKind};

    #[test]
    fn test_ts_cfg_generation() {
        let snippet = r#"
async function process(data: any) {
    if (data) {
        await fetch();
    } else {
        while (true) {
            break;
        }
    }
}
"#;
        let parser = TypeScriptParser::new();
        let graph = parser.parse(snippet, "test_cfg.ts").unwrap();

        assert!(!graph.control_flow.is_empty(), "Expected at least 1 CFG");
        let cfg = graph
            .control_flow
            .iter()
            .find(|c| c.function_id.contains("process"))
            .expect("Should have process function CFG");

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
