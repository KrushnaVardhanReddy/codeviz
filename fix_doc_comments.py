import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Add doc comments to run_watch, run_core, and RunResult

replacement = """/// Contains the results of a successful graph generation run.
pub struct RunResult {
    /// Number of files successfully parsed.
    pub parsed_count: usize,
    /// Total number of nodes in the generated graph.
    pub nodes: usize,
    /// Total number of edges in the generated graph.
    pub edges: usize,
}

/// Executes the core logic of the `run` or `check` command, separating parsing and checking from CLI formatting.
pub fn run_core(
    run_args: &RunArgs,
    is_check: bool,
    quiet: bool,
) -> Result<(bool, RunResult, Vec<codeviz_core::parser::ParseError>), String> {"""

content = content.replace("""pub struct RunResult {
    pub parsed_count: usize,
    pub nodes: usize,
    pub edges: usize,
}

pub fn run_core(
    run_args: &RunArgs,
    is_check: bool,
    quiet: bool,
) -> Result<(bool, RunResult, Vec<codeviz_core::parser::ParseError>), String> {""", replacement)

replacement_watch = """/// Runs the `watch` command, recursively monitoring the target path for file modifications and automatically updating diagrams.
pub fn run_watch(run_args: &RunArgs) -> Result<bool, String> {"""

content = content.replace("pub fn run_watch(run_args: &RunArgs) -> Result<bool, String> {", replacement_watch)

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)

with open('codeviz-cli/tests/watch_test.rs', 'r') as f:
    test_content = f.read()

test_content = test_content.replace("""    let mut cmd = Command::cargo_bin("codeviz-cli").unwrap();
    cmd.arg("watch").arg("--path").arg(dir.path()).arg("--output").arg("out.md");\n\n""", "")

with open('codeviz-cli/tests/watch_test.rs', 'w') as f:
    f.write(test_content)
