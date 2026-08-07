# Spec: LanguageParser Trait

## Purpose
`LanguageParser` is the interface every language adapter must implement.
It decouples language-specific parsing from the core rendering pipeline.

---

## Trait Definition
```rust
pub trait LanguageParser: Send + Sync {
    /// e.g. "python", "typescript", "go"
    fn language_name(&self) -> &str;

    /// File extensions handled, e.g. ["py"] or ["ts", "tsx", "js"]
    fn supported_extensions(&self) -> &[&str];

    /// Parse source code into a CodeGraph.
    /// - `source`: full text content of the file
    /// - `file_path`: path relative to source_root (used as node id prefix)
    fn parse(&self, source: &str, file_path: &str) -> Result<CodeGraph, ParseError>;
}
```

---

## ParseError
```rust
pub struct ParseError {
    pub message:   String,
    pub file_path: String,
    pub line:      Option<u32>,  // None if error is not line-specific
}
```

---

## LanguageRegistry
```rust
pub struct LanguageRegistry {
    parsers: Vec<Box<dyn LanguageParser>>,
}

impl LanguageRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, parser: Box<dyn LanguageParser>);

    /// Dispatches to the parser whose extensions include `file_path`'s extension.
    /// Returns Err if no parser matches.
    pub fn parse_file(&self, file_path: &str, source: &str) -> Result<CodeGraph, ParseError>;

    /// Returns the parser for a given extension, or None.
    pub fn find_parser(&self, extension: &str) -> Option<&dyn LanguageParser>;
}
```

---

## Constraints
- If multiple parsers claim the same extension, the **last registered** parser wins.
- A parser must never panic — all errors must be returned via `ParseError`.
- Parsers must be `Send + Sync` to allow parallel file parsing in future.

---

## Acceptance Criteria
- `parse_file("foo.py", source)` dispatches to the Python parser if registered.
- `parse_file("foo.unknown", source)` returns `Err(ParseError { message: "No parser for extension: unknown", ... })`.
- Registering two parsers for `.py` — the second takes priority.
