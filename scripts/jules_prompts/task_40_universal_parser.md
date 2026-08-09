TASK: T40 — Query-Based Universal Language Parser (codeviz-langs)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Build a `codeviz-langs` crate that implements a Generic Parser Engine.
It reads TOML language definition files and uses tree-sitter queries to extract
nodes and edges for any language — without requiring a bespoke Rust parser.

Ship with 6 language definition files: Ruby, Swift, C#, PHP, Dart, Lua.
Register all parsers in the `LanguageRegistry` via a single `register_all()` call.

Files to Create:
- `codeviz-langs/Cargo.toml`
- `codeviz-langs/src/lib.rs`
- `codeviz-langs/src/engine.rs`   (GenericParser struct)
- `codeviz-langs/src/config.rs`   (LangConfig serde struct)
- `codeviz-langs/src/grammar_map.rs` (grammar name → tree_sitter::Language)
- `codeviz-langs/languages/ruby.toml`
- `codeviz-langs/languages/swift.toml`
- `codeviz-langs/languages/csharp.toml`
- `codeviz-langs/languages/php.toml`
- `codeviz-langs/languages/dart.toml`
- `codeviz-langs/languages/lua.toml`

Files to Modify:
- `Cargo.toml` (add `codeviz-langs` to workspace members)
- `codeviz-cli/Cargo.toml` (add `codeviz-langs` dependency)
- `codeviz-cli/src/main.rs` (call `codeviz_langs::register_all(&mut registry)`)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/universal_parser.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Do NOT change the existing Python and TypeScript parser crates.
  The generic engine supplements them — it does not replace them.
- TOML files must be embedded into the binary using `include_str!()`.
  Do NOT use runtime file I/O to load language definitions.
- The `grammar_map.rs` must have an explicit match arm for every supported
  grammar name string. Return `Err(ParseError)` for unknown grammar names.
- No `unwrap()` in parser logic. Return `Result<CodeGraph, ParseError>`.
- DO NOT use the unstable `let_chains` feature! You must use stable Rust 2021.
  Use nested `if let` or the `matches!` macro instead.
- Ensure `cargo clippy --all -- -D warnings` and `cargo test --all` pass.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- For `grammar_map.rs`, use a simple `match` on the grammar name string:
  ```rust
  pub fn get_language(name: &str) -> Result<tree_sitter::Language, ParseError> {
      match name {
          "tree-sitter-ruby"    => Ok(tree_sitter_ruby::language()),
          "tree-sitter-swift"   => Ok(tree_sitter_swift::language()),
          _ => Err(ParseError { message: format!("Unknown grammar: {}", name), ... })
      }
  }
  ```
- For running a tree-sitter query, use `tree_sitter::QueryCursor`:
  ```rust
  let query = tree_sitter::Query::new(&language, query_str)?;
  let mut cursor = tree_sitter::QueryCursor::new();
  let matches = cursor.matches(&query, root_node, source_bytes);
  ```
- The `@name` capture index can be found with `query.capture_index_for_name("name")`.
- For imports, the `@path` capture gives you the raw string text (e.g., `"rails"`).
  Use `node.utf8_text(source_bytes)?` to get the string value.
- Write at least one unit test per language TOML that parses a tiny code snippet
  and asserts the correct number of nodes/edges are produced.
- In `lib.rs`, implement `register_all` as:
  ```rust
  pub fn register_all(registry: &mut LanguageRegistry) {
      for lang in all_parsers() {
          registry.register(Box::new(lang));
      }
  }
  ```
  where `all_parsers()` returns `Vec<GenericParser>` by loading each embedded TOML.
