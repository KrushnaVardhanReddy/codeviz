use codeviz_core::render::mermaid::DiagramKind;
use codeviz_core::render::mermaid::MermaidRenderer;
use std::env;

/// Arguments for the `run` command.
#[derive(Debug, PartialEq)]
pub struct RunArgs {
    /// Directory to scan recursively for source files.
    pub path: String,
    /// Markdown file to inject the diagram into.
    pub output: String,
    /// Diagram type to generate.
    pub diagram: DiagramKind,
    /// Maximum graph depth (unlimited if None).
    pub depth: Option<usize>,
}

impl Default for RunArgs {
    fn default() -> Self {
        Self {
            path: ".".to_string(),
            output: "README.md".to_string(),
            diagram: DiagramKind::ModuleGraph,
            depth: None,
        }
    }
}

/// Parses diagram kind from a string.
fn parse_diagram_kind(s: &str) -> Result<DiagramKind, String> {
    match s {
        "module" => Ok(DiagramKind::ModuleGraph),
        "call" => Ok(DiagramKind::CallGraph),
        "class" => Ok(DiagramKind::ClassDiagram),
        _ => Err(format!("Invalid diagram kind: {}", s)),
    }
}

/// Parses CLI arguments into a `RunArgs` struct.
pub fn parse_run_args(args: &[String]) -> Result<RunArgs, String> {
    let mut run_args = RunArgs::default();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                if i + 1 < args.len() {
                    run_args.path = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Missing argument for --path".to_string());
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    run_args.output = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Missing argument for --output".to_string());
                }
            }
            "--diagram" => {
                if i + 1 < args.len() {
                    run_args.diagram = parse_diagram_kind(&args[i + 1])?;
                    i += 2;
                } else {
                    return Err("Missing argument for --diagram".to_string());
                }
            }
            "--depth" => {
                if i + 1 < args.len() {
                    let parsed_depth: usize = args[i + 1]
                        .parse()
                        .map_err(|_| format!("Invalid depth: {}", args[i + 1]))?;
                    run_args.depth = Some(parsed_depth);
                    i += 2;
                } else {
                    return Err("Missing argument for --depth".to_string());
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(run_args)
}

/// Prints the help message for the CLI.
pub fn print_help() {
    println!("codeviz --help");
    println!("Usage: codeviz <COMMAND> [OPTIONS]");
    println!("Commands:");
    println!("  run     Parses source code and injects an updated diagram into a markdown file.");
    println!("Options:");
    println!("  --help  Print this help message");
}



use std::path::{Path, PathBuf};
use codeviz_core::{CodeGraph, GraphMeta, LanguageRegistry, inject_mermaid};
use codeviz_python::PythonParser;
use codeviz_typescript::TypeScriptParser;

/// Prunes a CodeGraph up to the specified max depth using BFS.
pub fn prune_graph(graph: &mut CodeGraph, max_depth: Option<usize>) {
    let depth = match max_depth {
        Some(d) if d > 0 => d,
        _ => return, // 0 or None means unlimited
    };

    let mut in_degrees: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for node in &graph.nodes {
        in_degrees.insert(node.id.clone(), 0);
    }
    for edge in &graph.edges {
        if let Some(count) = in_degrees.get_mut(&edge.to_id) {
            *count += 1;
        }
    }

    let mut queue: std::collections::VecDeque<(String, usize)> = std::collections::VecDeque::new();
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

    let mut roots = vec![];
    for (id, count) in &in_degrees {
        if *count == 0 {
            roots.push(id.clone());
        }
    }

    if roots.is_empty() && !graph.nodes.is_empty() {
        roots.push(graph.nodes[0].id.clone());
    }

    for root in roots {
        queue.push_back((root.clone(), 0));
        visited.insert(root);
    }

    let mut adj: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for edge in &graph.edges {
        adj.entry(edge.from_id.clone()).or_default().push(edge.to_id.clone());
    }

    while let Some((node_id, current_depth)) = queue.pop_front() {
        if current_depth < depth
            && let Some(neighbors) = adj.get(&node_id) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor.clone(), current_depth + 1));
                    }
                }
            }
    }

    graph.nodes.retain(|n| visited.contains(&n.id));
    graph.edges.retain(|e| visited.contains(&e.from_id) && visited.contains(&e.to_id));
    graph.meta.node_count = graph.nodes.len();
    graph.meta.edge_count = graph.edges.len();
}

fn walk_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        let entries = std::fs::read_dir(dir).map_err(|e| format!("Failed to read dir {}: {}", dir.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                // Ignore dot directories
                if !path.file_name().unwrap_or_default().to_string_lossy().starts_with('.') {
                    files.extend(walk_dir(&path)?);
                }
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}


pub fn run_cli(args: Vec<String>) -> Result<(), String> {
    if args.iter().any(|arg| arg == "--help") {
        print_help();
        return Ok(());
    }

    if args.len() > 1 && args[1] == "serve" {
        if args.len() > 2 && args[2] == "--mcp" {
            return codeviz_mcp::start_mcp_server();
        } else {
            return Err("Unknown options for serve. Did you mean --mcp?".to_string());
        }
    }

    if args.len() > 1 && args[1] == "run" {
        let run_args = parse_run_args(&args[2..])?;

        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(PythonParser::new()));
        registry.register(Box::new(TypeScriptParser::new()));

        let files = walk_dir(Path::new(&run_args.path))?;

        let mut merged_graph = CodeGraph::new(GraphMeta {
            language: "mixed".to_string(),
            source_root: run_args.path.clone(),
            generated_at: "2024-01-01T00:00:00Z".to_string(), // In a real app, use chrono
            node_count: 0,
            edge_count: 0,
        });

        let mut parsed_count = 0;

        for file in files {
            // Check if file extension is supported before reading source
            if let Some(ext) = file.extension().and_then(|e| e.to_str())
                && registry.find_parser(ext).is_some()
                    && let Ok(source) = std::fs::read_to_string(&file)
                        && let Ok(graph) = registry.parse_file(&file.to_string_lossy(), &source) {
                            merged_graph.nodes.extend(graph.nodes);
                            merged_graph.edges.extend(graph.edges);
                            parsed_count += 1;
                        }
        }

        merged_graph.meta.node_count = merged_graph.nodes.len();
        merged_graph.meta.edge_count = merged_graph.edges.len();

        prune_graph(&mut merged_graph, run_args.depth);

        let renderer = MermaidRenderer::new();
        let mermaid_diagram = renderer.render(&merged_graph, run_args.diagram);

        let markdown = std::fs::read_to_string(&run_args.output)
            .map_err(|e| format!("Failed to read output file {}: {}", run_args.output, e))?;

        let updated_markdown = inject_mermaid(&markdown, &mermaid_diagram)
            .map_err(|e| format!("Failed to inject mermaid into {}: {}", run_args.output, e))?;

        std::fs::write(&run_args.output, updated_markdown)
            .map_err(|e| format!("Failed to write output file {}: {}", run_args.output, e))?;

        println!(
            "Successfully parsed {} files, generated diagram with {} nodes and {} edges. Output: {}",
            parsed_count,
            merged_graph.meta.node_count,
            merged_graph.meta.edge_count,
            run_args.output
        );
        return Ok(());
    }

    // Default error for unknown subcommands
    Err("Unknown command or options. Try --help".to_string())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run_cli(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_flag() {
        let args = vec!["codeviz".to_string(), "--help".to_string()];
        let result = run_cli(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_args_parsing() {
        let args = vec![
            "--path".to_string(), "src".to_string(),
            "--output".to_string(), "DOCS.md".to_string(),
            "--diagram".to_string(), "call".to_string(),
            "--depth".to_string(), "3".to_string()
        ];
        let run_args = parse_run_args(&args).unwrap();
        assert_eq!(run_args.path, "src");
        assert_eq!(run_args.output, "DOCS.md");
        assert_eq!(run_args.diagram, DiagramKind::CallGraph);
        assert_eq!(run_args.depth, Some(3));
    }

    #[test]
    fn test_missing_output_file() {
        let args = vec![
            "codeviz".to_string(), "run".to_string(),
            "--output".to_string(), "DOES_NOT_EXIST.md".to_string(),
        ];
        let result = run_cli(args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to read output file"));
    }

    #[test]
    fn test_depth_truncation() {
        use codeviz_core::{Node, Edge, EdgeKind, NodeKind};

        let meta = GraphMeta {
            language: "test".to_string(),
            source_root: "/".to_string(),
            generated_at: "".to_string(),
            node_count: 3,
            edge_count: 2,
        };
        let mut graph = CodeGraph::new(meta);

        // A -> B -> C
        graph.nodes.push(Node {
            id: "A".to_string(), label: "A".to_string(), kind: NodeKind::File, file_path: "A".to_string(), line: None, is_public: true
        });
        graph.nodes.push(Node {
            id: "B".to_string(), label: "B".to_string(), kind: NodeKind::File, file_path: "B".to_string(), line: None, is_public: true
        });
        graph.nodes.push(Node {
            id: "C".to_string(), label: "C".to_string(), kind: NodeKind::File, file_path: "C".to_string(), line: None, is_public: true
        });

        graph.edges.push(Edge { from_id: "A".to_string(), to_id: "B".to_string(), kind: EdgeKind::Imports });
        graph.edges.push(Edge { from_id: "B".to_string(), to_id: "C".to_string(), kind: EdgeKind::Imports });

        // Test max_depth = 1 (should keep A and B, drop C)
        let mut pruned = graph.clone();
        prune_graph(&mut pruned, Some(1));
        assert_eq!(pruned.nodes.len(), 2);
        assert!(pruned.nodes.iter().any(|n| n.id == "A"));
        assert!(pruned.nodes.iter().any(|n| n.id == "B"));
        assert!(!pruned.nodes.iter().any(|n| n.id == "C"));
        assert_eq!(pruned.edges.len(), 1);

        // Test max_depth = 2 (should keep all)
        let mut pruned2 = graph.clone();
        prune_graph(&mut pruned2, Some(2));
        assert_eq!(pruned2.nodes.len(), 3);
        assert_eq!(pruned2.edges.len(), 2);
    }
}
