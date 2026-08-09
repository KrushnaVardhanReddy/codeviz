use codeviz_core::render::OutputFormat;
use codeviz_core::render::dot::render_dot;
use codeviz_core::render::json::render_json;
use codeviz_core::render::mermaid::DiagramKind;
use codeviz_core::render::mermaid::MermaidRenderer;
use std::env;

/// Arguments for the `run` command.
#[derive(Debug, PartialEq)]
pub struct RunArgs {
    /// Config file path
    pub config_path: Option<String>,
    /// Directory to scan recursively for source files.
    pub path: String,
    /// Output format.
    pub format: OutputFormat,
    /// Markdown file to inject the diagram into, if any.
    pub output: Option<String>,
    /// Diagram type to generate.
    pub diagram: DiagramKind,
    /// Maximum graph depth (unlimited if None).
    pub depth: Option<usize>,
}

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct OutputTarget {
    pub file: String,
    pub diagram_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CodevizConfig {
    #[serde(default)]
    pub outputs: Vec<OutputTarget>,
}

fn load_config() -> Result<CodevizConfig, String> {
    let toml_file = std::env::var("CODEVIZ_CONFIG").unwrap_or_else(|_| "codeviz.toml".to_string());
    if let Ok(toml_str) = std::fs::read_to_string(&toml_file) {
        toml::from_str(&toml_str).map_err(|e| format!("Failed to parse codeviz.toml: {}", e))
    } else {
        Ok(CodevizConfig {
            outputs: vec![OutputTarget {
                file: "README.md".to_string(),
                diagram_type: "module".to_string(),
            }],
        })
    }
}

impl Default for RunArgs {
    fn default() -> Self {
        Self {
            config_path: None,
            path: ".".to_string(),
            format: OutputFormat::Mermaid,
            output: None,
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
    let mut config_path = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--config" {
            if i + 1 < args.len() {
                config_path = Some(args[i + 1].clone());
                break;
            } else {
                return Err("Missing argument for --config".to_string());
            }
        }
        i += 1;
    }

    let config = match &config_path {
        Some(path) => codeviz_core::Config::load_from_file(std::path::Path::new(path))?,
        None => codeviz_core::Config::load_from_dir(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")))?,
    };

    let mut run_args = RunArgs {
        config_path,
        path: ".".to_string(),
        output: config.output.targets.first().cloned().unwrap_or_else(|| "README.md".to_string()),
        diagram: parse_diagram_kind(&config.graph.diagram_type).unwrap_or(DiagramKind::ModuleGraph),
        depth: if config.graph.max_depth == 0 { None } else { Some(config.graph.max_depth) },
    };

    i = 0;
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
            "--config" => {
                i += 2;
            }
            "--output" => {
                if i + 1 < args.len() {
                    let val = args[i + 1].clone();
                    if val.to_lowercase() == "mermaid"
                        || val.to_lowercase() == "json"
                        || val.to_lowercase() == "dot"
                    {
                        run_args.format = val.parse().unwrap();
                        run_args.output = None;
                    } else {
                        run_args.format = OutputFormat::Mermaid;
                        run_args.output = Some(val);
                    }
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
    println!("  serve   Starts the MCP tool server.");
    println!("Options:");
    println!("  --help  Print this help message");
}

use codeviz_core::{CodeGraph, GraphMeta, LanguageRegistry, inject_mermaid};
use codeviz_go::GoParser;
use codeviz_java::JavaParser;
use codeviz_kotlin::KotlinParser;
use codeviz_python::PythonParser;
use codeviz_rust::RustLangParser;
use codeviz_typescript::TypeScriptParser;
use std::path::{Path, PathBuf};

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
        adj.entry(edge.from_id.clone())
            .or_default()
            .push(edge.to_id.clone());
    }

    while let Some((node_id, current_depth)) = queue.pop_front() {
        if current_depth < depth
            && let Some(neighbors) = adj.get(&node_id)
        {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back((neighbor.clone(), current_depth + 1));
                }
            }
        }
    }

    graph.nodes.retain(|n| visited.contains(&n.id));
    graph
        .edges
        .retain(|e| visited.contains(&e.from_id) && visited.contains(&e.to_id));
    graph.meta.node_count = graph.nodes.len();
    graph.meta.edge_count = graph.edges.len();
}

fn walk_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read dir {}: {}", dir.display(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Dir entry error: {}", e))?;
            let path = entry.path();
            if path.is_dir() {
                // Ignore dot directories
                if !path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .starts_with('.')
                {
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
        let is_mcp = args.iter().any(|arg| arg == "--mcp");
        if is_mcp {
            return codeviz_mcp::start_mcp_server();
        } else {
            return Err("serve requires --mcp flag".to_string());
        }
    }

    if args.len() > 1 && args[1] == "run" {
        let run_args = parse_run_args(&args[2..])?;

        let mut registry = LanguageRegistry::new();
        registry.register(Box::new(PythonParser::new()));
        registry.register(Box::new(TypeScriptParser::new()));
        registry.register(Box::new(GoParser::new()));
        registry.register(Box::new(RustLangParser::new()));
        registry.register(Box::new(JavaParser::new()));
        registry.register(Box::new(KotlinParser::new()));

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
                && let Ok(graph) = registry.parse_file(&file.to_string_lossy(), &source)
            {
                merged_graph.nodes.extend(graph.nodes);
                merged_graph.edges.extend(graph.edges);
                parsed_count += 1;
            }
        }

        merged_graph.meta.node_count = merged_graph.nodes.len();
        merged_graph.meta.edge_count = merged_graph.edges.len();

        prune_graph(&mut merged_graph, run_args.depth);

        match run_args.format {
            OutputFormat::Json => match render_json(&merged_graph) {
                Ok(json_str) => println!("{}", json_str),
                Err(e) => return Err(e),
            },
            OutputFormat::Dot => {
                let dot_str = render_dot(&merged_graph);
                println!("{}", dot_str);
            }
            OutputFormat::Mermaid => {
                let outputs = if let Some(out_file) = run_args.output {
                    let d_str = match run_args.diagram {
                        DiagramKind::ModuleGraph => "module",
                        DiagramKind::CallGraph => "call",
                        DiagramKind::ClassDiagram => "class",
                    };
                    vec![OutputTarget {
                        file: out_file,
                        diagram_type: d_str.to_string(),
                    }]
                } else {
                    let config = load_config()?;
                    config.outputs
                };

                let renderer = MermaidRenderer::new();
                for target in outputs {
                    let diagram_kind = parse_diagram_kind(&target.diagram_type)?;
                    let mermaid_diagram = renderer.render(&merged_graph, diagram_kind);

                    let markdown = match std::fs::read_to_string(&target.file) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to read output file {}: {}", target.file, e);
                            continue;
                        }
                    };

                    let updated_markdown = match inject_mermaid(&markdown, &mermaid_diagram) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to inject mermaid into {}: {}", target.file, e);
                            continue;
                        }
                    };

                    if let Err(e) = std::fs::write(&target.file, updated_markdown) {
                        eprintln!("Failed to write output file {}: {}", target.file, e);
                        continue;
                    }

                    println!(
                        "Successfully parsed {} files, generated diagram with {} nodes and {} edges. Output: {}",
                        parsed_count,
                        merged_graph.meta.node_count,
                        merged_graph.meta.edge_count,
                        target.file
                    );
                }
            }
        }

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
    fn test_parse_codeviz_config() {
        let toml_str = r#"
[[outputs]]
file         = "README.md"
diagram_type = "module"

[[outputs]]
file         = "docs/ARCHITECTURE.md"
diagram_type = "class"

[[outputs]]
file         = "docs/CALLGRAPH.md"
diagram_type = "call"
"#;
        let config: CodevizConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.outputs.len(), 3);
        assert_eq!(config.outputs[0].file, "README.md");
        assert_eq!(config.outputs[0].diagram_type, "module");
        assert_eq!(config.outputs[1].file, "docs/ARCHITECTURE.md");
        assert_eq!(config.outputs[1].diagram_type, "class");
        assert_eq!(config.outputs[2].file, "docs/CALLGRAPH.md");
        assert_eq!(config.outputs[2].diagram_type, "call");
    }

    #[test]
    fn test_multiple_outputs_run() {
        // Setup mock environment
        std::fs::write(
            "mock_valid_1.md",
            "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->",
        )
        .unwrap();
        std::fs::write(
            "mock_valid_2.md",
            "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->",
        )
        .unwrap();
        std::fs::write("mock_invalid_f.md", "missing tags").unwrap();

        let toml_str = r#"
[[outputs]]
file         = "mock_valid_1.md"
diagram_type = "module"

[[outputs]]
file         = "mock_valid_2.md"
diagram_type = "module"
"#;
        let toml_file = format!("codeviz_{}.toml", std::process::id());
        std::fs::write(&toml_file, toml_str).unwrap();

        unsafe {
            std::env::set_var("CODEVIZ_CONFIG", &toml_file);
        }
        let args = vec![
            "codeviz".to_string(),
            "run".to_string(),
            "--path".to_string(),
            ".".to_string(),
        ];
        let res = run_cli(args);
        assert!(res.is_ok());

        // Clean up
        std::fs::remove_file("mock_valid_1.md").unwrap();
        std::fs::remove_file("mock_valid_2.md").unwrap();
        std::fs::remove_file(&toml_file).unwrap();


        std::fs::write(
            "mock_valid_f.md",
            "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->",
        )
        .unwrap();
        std::fs::write("mock_invalid_f.md", "missing tags").unwrap();

        let toml_str = r#"
[[outputs]]
file         = "mock_invalid_f.md"
diagram_type = "module"

[[outputs]]
file         = "mock_valid_f.md"
diagram_type = "module"
"#;
        let toml_file = format!("codeviz_{}.toml", std::process::id());
        std::fs::write(&toml_file, toml_str).unwrap();

        unsafe {
            std::env::set_var("CODEVIZ_CONFIG", &toml_file);
        }
        let args = vec![
            "codeviz".to_string(),
            "run".to_string(),
            "--path".to_string(),
            ".".to_string(),
        ];
        let res = run_cli(args);
        assert!(res.is_ok());

        let out = std::fs::read_to_string("mock_valid_f.md").unwrap();
        assert!(out.contains("graph TD\n"));

        std::fs::remove_file("mock_valid_f.md").unwrap();
        std::fs::remove_file("mock_invalid_f.md").unwrap();
        std::fs::remove_file(&toml_file).unwrap();
    }



    #[test]
    fn test_help_flag() {
        let args = vec!["codeviz".to_string(), "--help".to_string()];
        let result = run_cli(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_args_parsing() {
        let args = vec![
            "--path".to_string(),
            "src".to_string(),
            "--output".to_string(),
            "DOCS.md".to_string(),
            "--diagram".to_string(),
            "call".to_string(),
            "--depth".to_string(),
            "3".to_string(),
        ];
        let run_args = parse_run_args(&args).unwrap();
        assert_eq!(run_args.path, "src");
        assert_eq!(run_args.output, Some("DOCS.md".to_string()));
        assert_eq!(run_args.diagram, DiagramKind::CallGraph);
        assert_eq!(run_args.depth, Some(3));
    }

    #[test]
    fn test_cli_flag_precedence() {
        use std::io::Write;

        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
        [graph]
        diagram_type = "module"
        max_depth = 10
        "#).unwrap();

        let path_str = temp_file.path().to_string_lossy().to_string();

        let args = vec![
            "--config".to_string(), path_str,
            "--diagram".to_string(), "call".to_string() // CLI flag should override config
        ];

        let run_args = parse_run_args(&args).unwrap();
        assert_eq!(run_args.diagram, DiagramKind::CallGraph);
        assert_eq!(run_args.depth, Some(10));
    }

    #[test]
    fn test_missing_output_file() {
        let args = vec![
            "codeviz".to_string(),
            "run".to_string(),
            "--output".to_string(),
            "DOES_NOT_EXIST.md".to_string(),
        ];
        let result = run_cli(args);
        assert!(result.is_ok()); // Error is just logged now, it doesn't return err
    }

    #[test]
    fn test_depth_truncation() {
        use codeviz_core::{Edge, EdgeKind, Node, NodeKind};

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
            id: "A".to_string(),
            label: "A".to_string(),
            kind: NodeKind::File,
            file_path: "A".to_string(),
            line: None,
            is_public: true,
        });
        graph.nodes.push(Node {
            id: "B".to_string(),
            label: "B".to_string(),
            kind: NodeKind::File,
            file_path: "B".to_string(),
            line: None,
            is_public: true,
        });
        graph.nodes.push(Node {
            id: "C".to_string(),
            label: "C".to_string(),
            kind: NodeKind::File,
            file_path: "C".to_string(),
            line: None,
            is_public: true,
        });

        graph.edges.push(Edge {
            from_id: "A".to_string(),
            to_id: "B".to_string(),
            kind: EdgeKind::Imports,
        });
        graph.edges.push(Edge {
            from_id: "B".to_string(),
            to_id: "C".to_string(),
            kind: EdgeKind::Imports,
        });

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
