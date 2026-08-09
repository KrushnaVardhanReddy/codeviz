use codeviz_core::render::OutputFormat;
use codeviz_core::render::dot::render_dot;
use codeviz_core::render::json::render_json;
use codeviz_core::render::mermaid::DiagramKind;
use codeviz_core::render::mermaid::MermaidRenderer;
use std::env;

/// Arguments for the `install-hook` command.
#[derive(Debug, PartialEq)]
pub struct InstallHookArgs {
    /// Directory to scan recursively for source files.
    pub path: String,
    /// Markdown file to inject the diagram into.
    pub output: String,
}

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

/// Normalizes whitespace in a diagram string for comparison.
/// - Trim leading/trailing whitespace per line
/// - Collapse multiple blank lines into one
/// - Ignore trailing newline differences
pub(crate) fn normalize_whitespace(s: &str) -> String {
    let mut result = String::new();
    let mut prev_blank = true; // Start with true to avoid leading blank lines

    for line in s.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !prev_blank {
                result.push('\n');
                prev_blank = true;
            }
        } else {
            result.push_str(trimmed);
            result.push('\n');
            prev_blank = false;
        }
    }

    // Remove trailing newline if present, to handle trailing newline differences
    while result.ends_with('\n') {
        result.pop();
    }

    // Also remove leading newline if present
    if result.starts_with('\n') {
        result.remove(0);
    }

    result
}

/// Extracts the content between CODEVIZ_START and CODEVIZ_END
fn extract_mermaid_from_markdown(markdown: &str) -> Result<String, String> {
    let start_tag = "<!-- CODEVIZ_START -->";
    let end_tag = "<!-- CODEVIZ_END -->";

    let start_idx = markdown.find(start_tag).ok_or_else(|| "Missing CODEVIZ_START tag".to_string())?;

    let search_slice = &markdown[start_idx + start_tag.len()..];
    let relative_end_idx = search_slice.find(end_tag).ok_or_else(|| "Missing CODEVIZ_END tag".to_string())?;

    let inner = &search_slice[..relative_end_idx];

    // We expect the inner content to be roughly:
    // \n```mermaid\n(DIAGRAM CONTENT)\n```\n
    // However, since we normalize everything before comparing, we can just return the raw inner content.
    Ok(inner.to_string())
}

/// Checks if a generated mermaid diagram matches what is currently in the output file.
pub(crate) fn check_diagram_up_to_date(
    _file_path: &str,
    markdown: &str,
    mermaid: &str,
) -> Result<bool, String> {
    let current_inner = match extract_mermaid_from_markdown(markdown) {
        Ok(c) => c,
        Err(_) => {
            println!("❌ Diagram is stale. Run codeviz run to update.");
            return Ok(false);
        }
    };

    let mut expected_inner = String::new();
    expected_inner.push_str("\n```mermaid\n");
    if !mermaid.is_empty() {
        expected_inner.push_str(mermaid);
        if !mermaid.ends_with('\n') {
            expected_inner.push('\n');
        }
    }
    expected_inner.push_str("```\n");

    let norm_current = normalize_whitespace(&current_inner);
    let norm_expected = normalize_whitespace(&expected_inner);

    if norm_current == norm_expected {
        println!("✅ Diagram is up-to-date.");
        Ok(true)
    } else {
        println!("❌ Diagram is stale. Run codeviz run to update.");

        let diff = similar::TextDiff::from_lines(&norm_current, &norm_expected);
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "+",
                similar::ChangeTag::Equal => " ",
            };
            print!("{}{}", sign, change);
        }
        println!();

        Ok(false)
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

/// Updates or creates the `.pre-commit-config.yaml` file to include the codeviz hook.
pub fn update_pre_commit_config(path: &Path) -> Result<(), String> {
    let config_path = path.join(".pre-commit-config.yaml");
    let hook_entry = r#"repos:
  - repo: local
    hooks:
      - id: codeviz
        name: Update architecture diagram
        entry: codeviz run --path ./src --output README.md
        language: system
        pass_filenames: false
        always_run: false
"#;

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        if content.contains("id: codeviz") {
            println!("Already configured in .pre-commit-config.yaml");
            return Ok(());
        } else {
            let mut new_content = content.trim_end().to_string();
            if !new_content.contains("repos:") {
                new_content.push_str("\nrepos:");
            }
            // Add hook entry without the duplicate 'repos:' root
            let entry_no_repo = r#"
  - repo: local
    hooks:
      - id: codeviz
        name: Update architecture diagram
        entry: codeviz run --path ./src --output README.md
        language: system
        pass_filenames: false
        always_run: false"#;
            new_content.push_str(entry_no_repo);
            new_content.push('\n');
            std::fs::write(&config_path, new_content).map_err(|e| e.to_string())?;
            println!("Updated .pre-commit-config.yaml");
        }
    } else {
        std::fs::write(&config_path, hook_entry).map_err(|e| e.to_string())?;
        println!("Created .pre-commit-config.yaml");
    }

    Ok(())
}

/// Updates or creates the `.git/hooks/pre-commit` file to run `codeviz check`.
pub fn update_git_hook(path: &Path) -> Result<(), String> {
    let hooks_dir = path.join(".git").join("hooks");
    if !hooks_dir.exists() {
        std::fs::create_dir_all(&hooks_dir)
            .map_err(|e| format!("Failed to create .git/hooks directory: {}", e))?;
    }

    let hook_path = hooks_dir.join("pre-commit");
    let hook_cmd = "codeviz check\n";

    if hook_path.exists() {
        let content = std::fs::read_to_string(&hook_path)
            .map_err(|e| format!("Failed to read hook: {}", e))?;
        if content.contains("codeviz check") {
            println!("Already configured in .git/hooks/pre-commit");
            return Ok(());
        } else {
            let mut new_content = content;
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str(hook_cmd);
            std::fs::write(&hook_path, new_content)
                .map_err(|e| format!("Failed to append to hook: {}", e))?;
            println!("Appended to .git/hooks/pre-commit");
        }
    } else {
        let content = format!("#!/bin/sh\n{}", hook_cmd);
        std::fs::write(&hook_path, content).map_err(|e| format!("Failed to create hook: {}", e))?;
        println!("Created .git/hooks/pre-commit");
    }

    // Set executable permission (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&hook_path)
            .map_err(|e| format!("Failed to read hook metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms)
            .map_err(|e| format!("Failed to set hook permissions: {}", e))?;
    }

    Ok(())
}

/// Updates or creates the specified Markdown file to include sentinel tags.
pub fn update_markdown_file(output: &str) -> Result<(), String> {
    let output_path = Path::new(output);
    let sentinel_tags =
        "\n---\n\n## Architecture\n\n<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->\n";

    if output_path.exists() {
        let content = std::fs::read_to_string(output_path)
            .map_err(|e| format!("Failed to read {}: {}", output, e))?;
        if content.contains("<!-- CODEVIZ_START -->") && content.contains("<!-- CODEVIZ_END -->") {
            println!("Sentinel tags already present in {}", output);
            return Ok(());
        } else {
            let mut new_content = content;
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            new_content.push_str(sentinel_tags);
            std::fs::write(output_path, new_content)
                .map_err(|e| format!("Failed to append to {}: {}", output, e))?;
            println!("Appended sentinel tags to {}", output);
        }
    } else {
        std::fs::write(output_path, sentinel_tags.trim_start())
            .map_err(|e| format!("Failed to create {}: {}", output, e))?;
        println!("Created {} with sentinel tags", output);
    }

    Ok(())
}

/// Runs the `install-hook` command logic.
pub fn run_install_hook(args: &InstallHookArgs) -> Result<(), String> {
    let path = Path::new(&args.path);

    // Check if pre-commit is installed
    match std::process::Command::new("which")
        .arg("pre-commit")
        .output()
    {
        Ok(output) if output.status.success() => {
            // pre-commit is installed
        }
        _ => {
            println!(
                "Warning: `pre-commit` not found in PATH. You may need to install it (e.g., `pip install pre-commit`) for the hook to run automatically."
            );
        }
    }

    update_pre_commit_config(path)?;
    update_git_hook(path)?;
    update_markdown_file(&args.output)?;

    println!("Install hook completed.");
    Ok(())
}

/// Parses the arguments for the `install-hook` command.
pub fn parse_install_hook_args(args: &[String]) -> Result<InstallHookArgs, String> {
    let mut hook_args = InstallHookArgs {
        path: "./src".to_string(),
        output: "README.md".to_string(),
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--path" => {
                if i + 1 < args.len() {
                    hook_args.path = args[i + 1].clone();
                    i += 1;
                } else {
                    return Err("--path requires a value".to_string());
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    hook_args.output = args[i + 1].clone();
                    i += 1;
                } else {
                    return Err("--output requires a value".to_string());
                }
            }
            _ => return Err(format!("Unknown option for install-hook: {}", args[i])),
        }
        i += 1;
    }

    Ok(hook_args)
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
        None => codeviz_core::Config::load_from_dir(
            &std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
        )?,
    };

    let mut run_args = RunArgs {
        config_path,
        path: ".".to_string(),
        format: OutputFormat::Mermaid,
        output: None,
        diagram: parse_diagram_kind(&config.graph.diagram_type).unwrap_or(DiagramKind::ModuleGraph),
        depth: if config.graph.max_depth == 0 {
            None
        } else {
            Some(config.graph.max_depth)
        },
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
    println!("  run          Parses source code and injects an updated diagram into a markdown file.");
    println!("  check        Checks if the generated diagram matches the output file without modifying it.");
    println!("  serve        Starts the MCP tool server.");
    println!("  install-hook Installs the pre-commit hook and markdown sentinel tags.");
    println!("Options:");
    println!("  --help       Print this help message");
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

pub fn run_cli(args: Vec<String>) -> Result<bool, String> {
    if args.iter().any(|arg| arg == "--help") {
        print_help();
        return Ok(true);
    }

    if args.len() > 1 && args[1] == "install-hook" {
        let install_hook_args = parse_install_hook_args(&args[2..])?;
        return run_install_hook(&install_hook_args).map(|_| true);
    }

    if args.len() > 1 && args[1] == "serve" {
        let is_mcp = args.iter().any(|arg| arg == "--mcp");
        if is_mcp {
            codeviz_mcp::start_mcp_server()?;
            return Ok(true);
        } else {
            return Err("serve requires --mcp flag".to_string());
        }
    }

    if args.len() > 1 && (args[1] == "run" || args[1] == "check") {
        let is_check = args[1] == "check";
        let run_args = parse_run_args(&args[2..])?;

        if is_check && run_args.format != OutputFormat::Mermaid {
            return Err("check command only supports Mermaid output format".to_string());
        }

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

        let mut architecture_violations = false;
        if is_check {
            let config = match &run_args.config_path {
                Some(path) => codeviz_core::Config::load_from_file(std::path::Path::new(path)).unwrap_or_default(),
                None => codeviz_core::Config::load_from_dir(&std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))).unwrap_or_default(),
            };
            for edge in &merged_graph.edges {
                if edge.kind == codeviz_core::EdgeKind::Imports {
                    for rule in &config.architecture.rules {
                        if edge.from_id.contains(&rule.from) && edge.to_id.contains(&rule.cannot_import) {
                            println!(
                                "❌ Architectural violation: {} cannot import {}",
                                edge.from_id, edge.to_id
                            );
                            architecture_violations = true;
                        }
                    }
                }
            }
        }

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
                let mut all_up_to_date = !architecture_violations;

                for target in outputs {
                    let diagram_kind = parse_diagram_kind(&target.diagram_type)?;
                    let mermaid_diagram = renderer.render(&merged_graph, diagram_kind);

                    let markdown = match std::fs::read_to_string(&target.file) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!("Failed to read output file {}: {}", target.file, e);
                            if is_check {
                                all_up_to_date = false;
                            }
                            continue;
                        }
                    };

                    if is_check {
                        match check_diagram_up_to_date(&target.file, &markdown, &mermaid_diagram) {
                            Ok(true) => {}
                            Ok(false) => {
                                all_up_to_date = false;
                            }
                            Err(e) => {
                                eprintln!("Error checking diagram in {}: {}", target.file, e);
                                all_up_to_date = false;
                            }
                        }
                    } else {
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

                if is_check {
                    return Ok(all_up_to_date);
                }
            }
        }

        return Ok(true);
    }

    // Default error for unknown subcommands
    Err("Unknown command or options. Try --help".to_string())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match run_cli(args) {
        Ok(true) => Ok(()),
        Ok(false) => {
            std::process::exit(1);
        }
        Err(e) => Err(e),
    }
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
        writeln!(
            temp_file,
            r#"
        [graph]
        diagram_type = "module"
        max_depth = 10
        "#
        )
        .unwrap();

        let path_str = temp_file.path().to_string_lossy().to_string();

        let args = vec![
            "--config".to_string(),
            path_str,
            "--diagram".to_string(),
            "call".to_string(), // CLI flag should override config
        ];

        let run_args = parse_run_args(&args).unwrap();
        assert_eq!(run_args.diagram, DiagramKind::CallGraph);
        assert_eq!(run_args.depth, Some(10));
    }

    #[test]
    fn test_install_hook_no_existing_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let args = InstallHookArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("README.md")
                .to_string_lossy()
                .to_string(),
        };

        // Ensure git hook dir doesn't fail
        std::fs::create_dir_all(temp_dir.path().join(".git").join("hooks")).unwrap();

        let res = super::run_install_hook(&args);
        assert!(res.is_ok());

        let config_path = temp_dir.path().join(".pre-commit-config.yaml");
        assert!(config_path.exists());
        let content = std::fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("id: codeviz"));
        assert!(content.contains("repos:"));

        let hook_path = temp_dir
            .path()
            .join(".git")
            .join("hooks")
            .join("pre-commit");
        assert!(hook_path.exists());
        let hook_content = std::fs::read_to_string(&hook_path).unwrap();
        assert!(hook_content.contains("codeviz check"));

        let output_path = temp_dir.path().join("README.md");
        assert!(output_path.exists());
        let md_content = std::fs::read_to_string(&output_path).unwrap();
        assert!(md_content.contains("<!-- CODEVIZ_START -->"));
    }

    #[test]
    fn test_install_hook_existing_config_without_entry() {
        let temp_dir = tempfile::tempdir().unwrap();
        let args = InstallHookArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("README.md")
                .to_string_lossy()
                .to_string(),
        };

        // Create initial config without codeviz entry
        let config_path = temp_dir.path().join(".pre-commit-config.yaml");
        std::fs::write(&config_path, "repos:\n  - repo: some-other-repo\n").unwrap();

        std::fs::create_dir_all(temp_dir.path().join(".git").join("hooks")).unwrap();

        let res = super::run_install_hook(&args);
        assert!(res.is_ok());

        let content = std::fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("some-other-repo"));
        assert!(content.contains("id: codeviz"));
        // Should only have one 'repos:'
        assert_eq!(content.matches("repos:").count(), 1);
    }

    #[test]
    fn test_install_hook_existing_config_with_entry() {
        let temp_dir = tempfile::tempdir().unwrap();
        let args = InstallHookArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            output: temp_dir
                .path()
                .join("README.md")
                .to_string_lossy()
                .to_string(),
        };

        let initial_content = "repos:\n  - repo: local\n    hooks:\n      - id: codeviz\n";
        let config_path = temp_dir.path().join(".pre-commit-config.yaml");
        std::fs::write(&config_path, initial_content).unwrap();

        std::fs::create_dir_all(temp_dir.path().join(".git").join("hooks")).unwrap();

        let res = super::run_install_hook(&args);
        assert!(res.is_ok());

        // Content should be unchanged
        let content = std::fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, initial_content);
    }

    #[test]
    fn test_install_hook_sentinel_tags() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_file = temp_dir.path().join("README.md");
        let args = InstallHookArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            output: output_file.to_string_lossy().to_string(),
        };

        std::fs::create_dir_all(temp_dir.path().join(".git").join("hooks")).unwrap();

        // 1. Create file and append tags
        std::fs::write(&output_file, "Some existing content.").unwrap();
        let res = super::run_install_hook(&args);
        assert!(res.is_ok());

        let content1 = std::fs::read_to_string(&output_file).unwrap();
        assert!(content1.starts_with("Some existing content.\n"));
        assert!(content1.contains("<!-- CODEVIZ_START -->"));

        // 2. Run again, verify tags are not duplicated
        let res2 = super::run_install_hook(&args);
        assert!(res2.is_ok());

        let content2 = std::fs::read_to_string(&output_file).unwrap();
        assert_eq!(content1, content2);
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
    fn test_check_normalization() {
        let text1 = " \n \n  A  \nB\n \n";
        let text2 = "A\nB";
        assert_eq!(normalize_whitespace(text1), normalize_whitespace(text2));
    }

    #[test]
    fn test_check_up_to_date() {
        let markdown = "Header\n<!-- CODEVIZ_START -->\n```mermaid\ngraph TD\nA --> B\n```\n<!-- CODEVIZ_END -->\nFooter";
        let mermaid = "graph TD\nA --> B";

        let result = check_diagram_up_to_date("dummy.md", markdown, mermaid);
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_check_stale_diagram() {
        let markdown = "Header\n<!-- CODEVIZ_START -->\n```mermaid\ngraph TD\nA --> B\n```\n<!-- CODEVIZ_END -->\nFooter";
        let mermaid = "graph TD\nA --> B\nB --> C";

        let result = check_diagram_up_to_date("dummy.md", markdown, mermaid);
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_check_never_writes() {
        use std::io::Write;
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->").unwrap();
        let path_str = temp_file.path().to_string_lossy().to_string();

        let initial_modified = std::fs::metadata(&path_str).unwrap().modified().unwrap();

        // Ensure some time passes so modified time would change if it were written
        std::thread::sleep(std::time::Duration::from_millis(10));

        let args = vec![
            "codeviz".to_string(),
            "check".to_string(), // use check command
            "--path".to_string(),
            ".".to_string(),
            "--output".to_string(),
            path_str.clone(),
        ];

        // We expect it might fail the check if the diagram doesn't match the empty tags,
        // but it should NOT write to the file.
        let _ = run_cli(args);

        let final_modified = std::fs::metadata(&path_str).unwrap().modified().unwrap();
        assert_eq!(initial_modified, final_modified, "File should not be modified by check command");
    }

    #[test]
    fn test_architectural_rule_violation() {
        // 1. Create a mock source directory with two files that import each other
        let temp_dir = tempfile::tempdir().unwrap();
        let src_dir = temp_dir.path().join("src");
        std::fs::create_dir_all(&src_dir).unwrap();

        let mod_a_path = src_dir.join("moduleA.ts");
        let mod_b_path = src_dir.join("moduleB.ts");

        std::fs::write(&mod_a_path, "import { B } from './moduleB';").unwrap();
        std::fs::write(&mod_b_path, "export class B {}").unwrap();

        // 2. Create codeviz.toml with an architectural rule
        let toml_path = temp_dir.path().join("codeviz.toml");
        let toml_content = r#"
        [[architecture.rules]]
        from = "moduleA"
        cannot_import = "moduleB"
        "#;
        std::fs::write(&toml_path, toml_content).unwrap();

        let doc_path = temp_dir.path().join("DOCS.md");
        std::fs::write(&doc_path, "<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->").unwrap();

        let args = vec![
            "codeviz".to_string(),
            "check".to_string(),
            "--path".to_string(),
            temp_dir.path().to_string_lossy().to_string(),
            "--config".to_string(),
            toml_path.to_string_lossy().to_string(),
            "--output".to_string(),
            doc_path.to_string_lossy().to_string(),
        ];

        // 3. Run check - should return Ok(false) due to violation
        let result = run_cli(args);
        assert_eq!(result, Ok(false));
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
