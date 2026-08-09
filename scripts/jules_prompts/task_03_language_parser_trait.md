# Jules Task 03 — LanguageParser Trait

## Objective
Define the `LanguageParser` trait in `codeviz-core`. Every language adapter must implement this.

## Files to Create/Modify
- `codeviz-core/src/parser.rs`
- `codeviz-core/src/lib.rs` (re-export)

## Requirements
```rust
pub trait LanguageParser {
    /// Human-readable name, e.g. "python", "typescript"
    fn language_name(&self) -> &str;

    /// File extensions this parser handles, e.g. ["py"]
    fn supported_extensions(&self) -> &[&str];

    /// Parse source code string into a CodeGraph.
    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError>;
}

pub struct ParseError {
    pub message: String,
    pub file_path: String,
    pub line: Option<u32>,
}
```

Also add a `LanguageRegistry` struct that:
- Holds a `Vec<Box<dyn LanguageParser>>`
- Has `register(parser)`, `parse_file(path, source)` (dispatches by extension)
- Returns `Err(ParseError)` if no parser matches the extension

## Unit Tests
Write unit tests using a mock `LanguageParser` implementation that:
- Returns a fixed `CodeGraph` for any input
- Verifies `LanguageRegistry::parse_file` dispatches correctly by extension
- Verifies an unknown extension returns the expected error


═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use stable Rust 2021 edition. Do NOT use unstable features like `let_chains`.
- Always run `cargo test --all` and `cargo clippy --all -- -D warnings` before completing the task.
