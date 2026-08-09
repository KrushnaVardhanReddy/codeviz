# Spec: Query-Based Universal Language Parser (Phase 17)

## Overview
Instead of writing a bespoke Rust parser for every programming language, CodeViz
will ship a **Generic Parser Engine** that reads a small TOML language definition
file and executes tree-sitter queries to extract nodes and edges automatically.

Community contributors can add support for any language (Ruby, PHP, Swift, C#,
Dart, etc.) by writing a single `.toml` file — no Rust knowledge required.

---

## Architecture

### Language Definition Files
Stored in `codeviz-core/languages/*.toml` (shipped with the binary via `include_str!`).

### Generic Parser Engine
A single Rust crate `codeviz-langs` that:
1. Loads all `*.toml` files from the `languages/` directory.
2. For each file, builds a `LanguageParser` implementation dynamically.
3. Registers itself with the `LanguageRegistry`.

---

## Language Definition File Format

```toml
[language]
name       = "ruby"
extensions = ["rb"]
# tree-sitter crate to use. Must be a known, pre-compiled grammar.
grammar    = "tree-sitter-ruby"

[queries]
# Tree-sitter S-expression query strings.
# Capture @name is required for the node label.
# Capture @node is required for line number extraction.
# Capture @async is optional — if present, the Function is marked is_async=true.

functions = """
  (method
    name: (identifier) @name
  ) @node
"""

async_functions = """
  (singleton_method
    name: (identifier) @name
  ) @node
"""

classes = """
  (class
    name: (constant) @name
  ) @node
"""

interfaces = """
  (module
    name: (constant) @name
  ) @node
"""

# For imports, capture the module path string as @path
imports = """
  (call
    method: (identifier) @method (#eq? @method "require")
    arguments: (argument_list
      (string (string_content) @path))
  )
"""

# For inheritance, capture @from (child class) and @to (parent class)
inheritance = """
  (class
    name: (constant) @from
    superclass: (constant) @to
  )
"""

# For public/exported symbols — capture @name
exports = """
  (assignment
    left: (constant) @name
  )
"""
```

---

## Pre-compiled Grammar Registry

Tree-sitter grammars require compilation. We cannot dynamically link arbitrary
grammars at runtime in a safe CLI binary. Therefore, the `codeviz-langs` crate
will ship with a **fixed set of pre-compiled grammars** as Rust `extern` bindings.

### Supported Grammars (Phase 17)
| Grammar | Languages |
|---|---|
| `tree-sitter-ruby` | `.rb` |
| `tree-sitter-swift` | `.swift` |
| `tree-sitter-c-sharp` | `.cs` |
| `tree-sitter-php` | `.php` |
| `tree-sitter-dart` | `.dart` |
| `tree-sitter-lua` | `.lua` |

**Adding a new grammar:** Add the crate to `codeviz-langs/Cargo.toml` and add a
`(grammar_name, language_fn)` entry to the `GRAMMAR_REGISTRY` in `engine.rs`.

---

## Module Structure

```
codeviz-langs/
├── Cargo.toml
├── src/
│   ├── lib.rs          (re-exports)
│   ├── engine.rs       (GenericParser: loads TOML, runs queries)
│   ├── config.rs       (LangConfig struct: serde-deserialized TOML)
│   └── grammar_map.rs  (maps grammar name → tree_sitter::Language fn)
└── languages/
    ├── ruby.toml
    ├── swift.toml
    ├── csharp.toml
    ├── php.toml
    ├── dart.toml
    └── lua.toml
```

---

## `LangConfig` Struct

```rust
#[derive(Deserialize)]
pub struct LangConfig {
    pub language: LanguageMeta,
    pub queries: QueryConfig,
}

#[derive(Deserialize)]
pub struct LanguageMeta {
    pub name: String,
    pub extensions: Vec<String>,
    pub grammar: String,
}

#[derive(Deserialize)]
pub struct QueryConfig {
    pub functions: Option<String>,
    pub async_functions: Option<String>,
    pub classes: Option<String>,
    pub interfaces: Option<String>,
    pub imports: Option<String>,
    pub inheritance: Option<String>,
    pub exports: Option<String>,
}
```

---

## Engine Behavior

For each query field defined in the TOML:
1. Run `tree_sitter::Query::new(language, query_str)` on the parsed AST.
2. Iterate over all `QueryMatch` results.
3. Extract:
   - `@name` capture → `Node.label` and `Node.id`
   - `@node` capture → `Node.line` (start row of the captured node)
   - `@path` capture → `Edge.to_id` (for imports)
   - `@from` / `@to` captures → `Edge.from_id` / `Edge.to_id` (for inheritance)
4. Add the resulting `Node` or `Edge` to the `CodeGraph`.

---

## Integration with LanguageRegistry

In `codeviz-cli/src/main.rs`, replace individual `register()` calls with:
```rust
codeviz_langs::register_all(&mut registry);
```
This registers ALL language parsers (both hand-written and generic ones) in one call.

---

## Acceptance Criteria
- [ ] `GenericParser` implements `LanguageParser` trait.
- [ ] All 6 TOML language files ship with the binary (via `include_str!`).
- [ ] `cargo clippy --all -- -D warnings` passes.
- [ ] `cargo test --all` passes.
- [ ] Running `codeviz run --path ./test_ruby_project --output mermaid` produces a valid Mermaid diagram for a Ruby project.
- [ ] A contributor can add a new language by adding one `.toml` file and one line in `grammar_map.rs`.
