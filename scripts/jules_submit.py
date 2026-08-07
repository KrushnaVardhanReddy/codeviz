#!/usr/bin/env python3
"""
Jules Batch Submitter for CodeViz
===================================
Submits coding tasks to Jules (https://jules.google.com) for the CodeViz project.
Tasks map directly to the CodeViz roadmap phases.

Usage:
  python scripts/jules_submit.py --list
  python scripts/jules_submit.py --task 1
  python scripts/jules_submit.py --batch 1
  python scripts/jules_submit.py --batch 2 --branch feature/python-parser
"""

import json
import urllib.request
import urllib.error
import sys
import os

# ─────────────────────────────────────────────────────────────
# Auth
# ─────────────────────────────────────────────────────────────

def _load_api_key():
    key = os.environ.get("JULES_API_KEY")
    if key:
        return key
    for envfile in [".env.local", ".env"]:
        repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        path = os.path.join(repo_root, envfile)
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("JULES_API_KEY="):
                        return line.split("=", 1)[1].strip()
    print("❌ JULES_API_KEY not found.")
    print("   Add it to .env.local: JULES_API_KEY=<your-key>")
    sys.exit(1)

API_KEY  = _load_api_key()
API_URL  = "https://jules.googleapis.com/v1alpha/sessions"

# ─────────────────────────────────────────────────────────────
# Project config
# ─────────────────────────────────────────────────────────────

REPO_ROOT   = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO_SOURCE = "sources/github/KrushnaVardhanReddy/codeviz"

BRANCH = "main"
if "--branch" in sys.argv:
    idx = sys.argv.index("--branch")
    if idx + 1 < len(sys.argv):
        BRANCH = sys.argv[idx + 1]

# ─────────────────────────────────────────────────────────────
# Safety rules (prepended to every prompt)
# ─────────────────────────────────────────────────────────────

SAFETY_RULES = """
MANDATORY RULES — VIOLATION = REJECTED PR:
1. NEVER stub, mock, or TODO core logic. Every function must be fully implemented.
2. All Rust code must compile cleanly with `cargo build` (no warnings allowed).
3. Every public function must have a doc-comment (`///`).
4. No `unwrap()` in library code — use `?` or explicit error handling.
5. Commit message must start with "jules: " prefix.
6. Include comprehensive unit tests (`#[cfg(test)] mod tests { ... }`) for all new logic.

Project: CodeViz
Tech stack: Rust (workspace), Tree-sitter, wasm-pack, JSON-RPC (MCP)
Architecture: codeviz-core / codeviz-cli / codeviz-wasm / codeviz-mcp crates
""".strip()

# ─────────────────────────────────────────────────────────────
# Prompt loader
# ─────────────────────────────────────────────────────────────

def _load_prompt(slug: str) -> str:
    """Load a prompt from scripts/jules_prompts/<slug>.md if it exists,
    otherwise fall back to the inline prompt defined in TASKS below."""
    full_path = os.path.join(REPO_ROOT, f"scripts/jules_prompts/{slug}.md")
    if os.path.exists(full_path):
        with open(full_path) as f:
            return f.read()
    return ""  # caller will use the inline prompt

# ─────────────────────────────────────────────────────────────
# Task definitions — one per roadmap item
# ─────────────────────────────────────────────────────────────

TASKS = {

    # ── Phase 0: Foundation ──────────────────────────────────

    1: {
        "name": "01 — Rust Workspace Init",
        "phase": "Phase 0: Foundation",
        "prompt": """
## Objective
Initialize the CodeViz Rust workspace with four crates.

## Files to Create
- `Cargo.toml` (workspace root)
- `codeviz-core/Cargo.toml` + `codeviz-core/src/lib.rs`
- `codeviz-cli/Cargo.toml` + `codeviz-cli/src/main.rs`
- `codeviz-wasm/Cargo.toml` + `codeviz-wasm/src/lib.rs`
- `codeviz-mcp/Cargo.toml` + `codeviz-mcp/src/lib.rs`

## Requirements
1. Workspace root `Cargo.toml` must declare all four members.
2. `codeviz-core` is a pure library crate (no OS I/O, no file system access).
3. `codeviz-cli` depends on `codeviz-core`. Entry point: `codeviz --help`.
4. `codeviz-wasm` has `crate-type = ["cdylib"]` for wasm-pack.
5. `codeviz-mcp` depends on `codeviz-core`. Stub the MCP server entry point.
6. Add `.gitignore` entries for `target/` and `*.wasm`.
7. `cargo build` must succeed with zero errors and zero warnings.
8. Add a basic dummy unit test in each crate's `lib.rs` or `main.rs` to verify test runners work.
""".strip()
    },

    2: {
        "name": "02 — CodeGraph IR Structs",
        "phase": "Phase 0: Foundation",
        "prompt": """
## Objective
Define the language-agnostic `CodeGraph` Intermediate Representation (IR) in `codeviz-core`.
This is the core data model — every language parser outputs a `CodeGraph`, every renderer consumes one.

## Files to Modify/Create
- `codeviz-core/src/graph.rs`
- `codeviz-core/src/lib.rs` (re-export `graph` module)
- `codeviz-core/Cargo.toml` (add `serde` + `serde_json` dependencies)

## Requirements
Define the following types with full `serde::Serialize/Deserialize` derives:

```
CodeGraph { nodes: Vec<Node>, edges: Vec<Edge>, meta: GraphMeta }

Node {
    id: String,          // unique, e.g. "src/parser.rs::parse_file"
    label: String,       // display name
    kind: NodeKind,
    file_path: String,
    line: Option<u32>,
}

NodeKind (enum): File | Module | Function { is_async: bool } | Class | Interface | Constant

Edge {
    from_id: String,
    to_id: String,
    kind: EdgeKind,
}

EdgeKind (enum): Imports | Calls | Inherits | Implements | Returns | Instantiates

GraphMeta {
    language: String,
    source_root: String,
    generated_at: String,  // ISO 8601 timestamp
}
```

Add unit tests in the same file that serialize and deserialize a minimal `CodeGraph` round-trip.
""".strip()
    },

    3: {
        "name": "03 — LanguageParser Trait",
        "phase": "Phase 0: Foundation",
        "prompt": """
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

Write unit tests with a mock `LanguageParser` implementation.
""".strip()
    },

    4: {
        "name": "04 — Mermaid Renderer",
        "phase": "Phase 0: Foundation",
        "prompt": """
## Objective
Build the Mermaid diagram renderer in `codeviz-core`. Takes a `CodeGraph`, outputs a Mermaid string.

## Files to Create/Modify
- `codeviz-core/src/render/mod.rs`
- `codeviz-core/src/render/mermaid.rs`
- `codeviz-core/src/lib.rs` (re-export)

## Requirements
Implement `MermaidRenderer` with three output modes selectable via `DiagramKind` enum:
1. `DiagramKind::ModuleGraph` — `graph TD` showing only `Imports` edges between `File`/`Module` nodes
2. `DiagramKind::CallGraph` — `flowchart TD` showing only `Calls` edges between `Function` nodes
3. `DiagramKind::ClassDiagram` — `classDiagram` showing `Inherits` and `Implements` edges

Rules:
- Node IDs in Mermaid output must be sanitized (replace `/`, `.`, `::` with `_`)
- If node count > 50, emit a Mermaid comment `%% WARNING: graph truncated at 50 nodes`
- Each output must be valid Mermaid syntax (test by inspecting string structure)

Write unit tests that render a known `CodeGraph` and assert the Mermaid string contains expected substrings.
""".strip()
    },

    5: {
        "name": "05 — Safe Markdown Injection",
        "phase": "Phase 0: Foundation",
        "prompt": """
## Objective
Implement the safe markdown injector in `codeviz-core`.
It updates ONLY the content between `<!-- CODEVIZ_START -->` and `<!-- CODEVIZ_END -->` sentinel tags.

## Files to Create/Modify
- `codeviz-core/src/inject.rs`
- `codeviz-core/src/lib.rs` (re-export)

## Requirements
```rust
pub fn inject_mermaid(markdown: &str, mermaid: &str) -> Result<String, InjectError>
```
- If both sentinel tags exist: replace content between them with the new Mermaid block.
- If no sentinel tags exist: return `Err(InjectError::MissingTags)`.
- If only one tag exists: return `Err(InjectError::MalformedTags)`.
- The injected block must be wrapped as: a newline, triple-backtick mermaid, the diagram, triple-backtick, and a trailing newline.
- All content OUTSIDE the sentinel tags must be 100% preserved (verified by test).

Write tests covering: normal injection, missing tags, malformed tags, and idempotency (inject twice = same result).
""".strip()
    },

    # ── Phase 1: Python Parser ────────────────────────────────

    6: {
        "name": "06 — Python Parser (Imports & Classes)",
        "phase": "Phase 1: Python",
        "prompt": """
## Objective
Implement the Python language parser in a new `codeviz-python` crate using Tree-sitter.

## Files to Create
- `codeviz-python/Cargo.toml`
- `codeviz-python/src/lib.rs`
- `codeviz-python/src/parser.rs`
- Update workspace `Cargo.toml` to include this crate

## Requirements
1. Add `tree-sitter` and `tree-sitter-python` as dependencies.
2. Implement `LanguageParser` for a `PythonParser` struct.
3. Extract the following into `CodeGraph`:
   - `import X` and `from X import Y` → `Edge { kind: EdgeKind::Imports }`
   - `class Foo(Bar):` → `Node { kind: NodeKind::Class }` + `Edge { kind: EdgeKind::Inherits }`
   - `def foo():` → `Node { kind: NodeKind::Function { is_async } }`
   - `@decorator` → add to node metadata as a label suffix
4. Dynamic imports (`importlib.import_module`) — skip silently, don't panic.
5. Circular imports — add both edges normally, let the renderer handle display.

## Test Requirements
- Parse the following snippet and assert the resulting `CodeGraph` has correct nodes/edges:
```python
import os
from pathlib import Path

class Animal:
    pass

class Dog(Animal):
    def bark(self): pass

async def main():
    d = Dog()
```
""".strip()
    },

    7: {
        "name": "07 — CLI: codeviz run (Python)",
        "phase": "Phase 1: Python",
        "prompt": """
## Objective
Wire the Python parser into the CLI adapter so `codeviz run` works end-to-end.

## Files to Modify
- `codeviz-cli/src/main.rs`
- `codeviz-cli/Cargo.toml` (add `codeviz-core`, `codeviz-python` deps)

## CLI Interface to Implement
```
codeviz run --path <dir> --output <file.md> [--diagram module|call|class] [--depth N]
```
- `--path`: directory to scan recursively for source files
- `--output`: markdown file to inject the diagram into (must have sentinel tags)
- `--diagram`: diagram type (default: `module`)
- `--depth`: maximum graph depth (default: unlimited)

## Requirements
1. Walk `--path` recursively, collect files by extension.
2. Dispatch each file to the `LanguageRegistry`.
3. Merge all per-file `CodeGraph`s into one.
4. Render via `MermaidRenderer` with the selected `DiagramKind`.
5. Inject into `--output` using `inject_mermaid`.
6. Print summary: files parsed, nodes, edges, output path.
7. Exit code 0 on success, 1 on any error.
8. Write unit tests for the CLI argument parsing and config merging.
""".strip()
    },

    8: {
        "name": "08 — GitHub Actions CI",
        "phase": "Phase 1: Python",
        "prompt": """
## Objective
Set up GitHub Actions CI for the CodeViz project.

## Files to Create
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml` (binary releases on tag push)

## ci.yml Requirements
Trigger: push to any branch, PR to main.
Jobs:
1. `test` — `cargo test --all` on ubuntu-latest, macos-latest, windows-latest
2. `lint` — `cargo clippy --all -- -D warnings` + `cargo fmt --check`
3. `wasm-build` — install `wasm-pack`, run `wasm-pack build codeviz-wasm --target web`

## release.yml Requirements
Trigger: push of tag `v*`.
Jobs:
1. Build Linux binary: `cargo build --release` → upload `codeviz` artifact
2. Build macOS binary (macos-latest) → upload `codeviz-macos` artifact
3. Create GitHub Release with both binaries attached
""".strip()
    },

    # ── Phase 2: TypeScript + WASM ────────────────────────────

    9: {
        "name": "09 — TypeScript/JavaScript Parser",
        "phase": "Phase 2: TypeScript + WASM",
        "prompt": """
## Objective
Implement the TypeScript/JavaScript language parser using Tree-sitter.

## Files to Create
- `codeviz-typescript/Cargo.toml`
- `codeviz-typescript/src/lib.rs`
- `codeviz-typescript/src/parser.rs`

## Requirements
Use `tree-sitter-typescript` (covers TS, TSX, and JS).
Extract into `CodeGraph`:
1. `import { X } from 'y'` (ESM) → `Edge { kind: Imports }`
2. `const x = require('y')` (CJS) → `Edge { kind: Imports }`
3. `class Foo extends Bar` → `Node { Class }` + `Edge { Inherits }`
4. `interface IFoo` → `Node { Interface }`
5. `function foo()` / arrow functions → `Node { Function }`
6. `export default` / named exports → mark node with `is_public: true` metadata

Handle gracefully (skip, don't panic):
- Dynamic `import('...')` calls
- Barrel files (`index.ts` re-exports)
- `.tsx` / `.jsx` JSX syntax (Tree-sitter handles it; just don't crash on JSX nodes)

Write tests for ESM imports, CJS require, class inheritance, and async functions.
""".strip()
    },

    10: {
        "name": "10 — WASM Adapter (wasm-pack)",
        "phase": "Phase 2: TypeScript + WASM",
        "prompt": """
## Objective
Build the WASM adapter so CodeViz can run in the browser.

## Files to Modify/Create
- `codeviz-wasm/src/lib.rs`
- `codeviz-wasm/Cargo.toml`
- `codeviz-wasm/README.md` (usage instructions for JS consumers)

## Requirements
1. Use `wasm-bindgen` to expose a JS-callable API:
```typescript
// Generated TypeScript signature
function parse(source: string, language: string, diagram_kind: string): string;
// Returns: Mermaid diagram string, or throws on error
```
2. `language` parameter: `"python"` | `"typescript"` | `"javascript"`
3. `diagram_kind` parameter: `"module"` | `"call"` | `"class"`
4. Bundle must compile with `wasm-pack build --target web`.
5. Bundle size must be < 3MB (enforce with a CI size check in `ci.yml`).
6. Add a `codeviz-wasm/index.html` demo page that loads the WASM and lets you paste code + see the diagram.
7. Add a basic `#[test]` in `lib.rs` testing the exported API with mock input.
""".strip()
    },

    # ── Phase 3: MCP Server ───────────────────────────────────

    11: {
        "name": "11 — MCP Server: Core & Tool Definitions",
        "phase": "Phase 3: MCP Server",
        "prompt": """
## Objective
Implement the MCP (Model Context Protocol) server adapter in `codeviz-mcp`.

## Files to Modify/Create
- `codeviz-mcp/src/lib.rs`
- `codeviz-mcp/src/server.rs`
- `codeviz-mcp/src/tools.rs`
- `codeviz-mcp/Cargo.toml`

## MCP Spec Reference
Implement MCP spec revision 2025-06-18 (JSON-RPC 2.0 over stdio).

## Tools to Implement (6 total)
Each tool must have a JSON Schema definition and a Rust handler:

1. `get_module_graph` — input: `{path: string}` → output: `{graph: CodeGraph, mermaid: string}`
2. `get_callers` — input: `{fn_name: string, path: string}` → output: `{callers: Vec<Node>}`
3. `get_callees` — input: `{fn_name: string, path: string}` → output: `{callees: Vec<Node>}`
4. `get_class_hierarchy` — input: `{path: string}` → output: `{mermaid: string}`
5. `find_entry_points` — input: `{path: string}` → output: `{nodes: Vec<Node>}` (nodes with no incoming Calls edges)
6. `explain_path` — input: `{from: string, to: string, path: string}` → output: `{path: Vec<Node>, exists: bool}`

## Requirements
- Server reads JSON-RPC from stdin, writes to stdout (one JSON object per line).
- Add `max_nodes: Option<usize>` cap (default: 200) to prevent huge responses.
- All errors must be returned as JSON-RPC error objects (not panics).
- Wire into CLI: `codeviz serve --mcp [--port N]`
- Write unit tests for each tool handler using mock `CodeGraph`s.
""".strip()
    },

    12: {
        "name": "12 — MCP Integration Tests",
        "phase": "Phase 3: MCP Server",
        "prompt": """
## Objective
Write integration tests for the MCP server using real CodeViz source code as the test corpus.

## Files to Create
- `codeviz-mcp/tests/integration_test.rs`
- `codeviz-mcp/tests/fixtures/sample_python.py`
- `codeviz-mcp/tests/fixtures/sample_typescript.ts`

## Requirements
1. Spawn `codeviz serve --mcp` as a subprocess.
2. Send JSON-RPC requests via stdin, read responses from stdout.
3. Test each of the 6 tools with the fixture files:
   - `get_module_graph` → assert at least 1 node and 1 edge returned
   - `get_callers` → assert correct caller nodes
   - `get_callees` → assert correct callee nodes
   - `get_class_hierarchy` → assert valid Mermaid classDiagram string returned
   - `find_entry_points` → assert at least 1 entry point found
   - `explain_path` → assert path exists between known connected nodes
4. Test error handling: unknown tool name → JSON-RPC error -32601
5. Create a `docs/mcp_config.md` showing how to add CodeViz to Claude Desktop, Cursor, and Continue.dev.
""".strip()
    },

    # ── Phase 4: Go Parser ────────────────────────────────────

    13: {
        "name": "13 — Go Parser",
        "phase": "Phase 4: Go",
        "prompt": """
## Objective
Implement the Go language parser using Tree-sitter.

## Files to Create
- `codeviz-go/Cargo.toml`
- `codeviz-go/src/lib.rs`
- `codeviz-go/src/parser.rs`

## Requirements
Extract into `CodeGraph`:
1. `import "pkg/path"` and `import ( ... )` blocks → `Edge { Imports }`
2. `type Foo struct { Bar }` (embedded struct) → `Edge { Inherits }`
3. `type Foo interface { ... }` → `Node { Interface }`
4. `func (f *Foo) Method()` → `Node { Function }` associated with struct node
5. Package-level `func main()` → mark as entry point

Graceful handling:
- `go.mod` file: read module name for correct import path resolution
- Build tags (`//go:build ...`): parse the default tag set only (no tag evaluation)
- Interface satisfaction is implicit in Go — do NOT attempt to infer it (document in comments)

Tests: parse a Go snippet with imports, a struct, an interface, and methods.
""".strip()
    },

    # ── Phase 5: Rust Parser ──────────────────────────────────

    14: {
        "name": "14 — Rust Parser",
        "phase": "Phase 5: Rust",
        "prompt": """
## Objective
Implement the Rust language parser using Tree-sitter (parsing Rust with Rust!).

## Files to Create
- `codeviz-rust-lang/Cargo.toml`  (name avoids conflict with std `rust` crate)
- `codeviz-rust-lang/src/lib.rs`
- `codeviz-rust-lang/src/parser.rs`

## Requirements
Extract into `CodeGraph`:
1. `use crate::module::Item` / `use super::...` / `extern crate foo` → `Edge { Imports }`
2. `struct Foo` / `enum Foo` → `Node { Class }` (treat both as class-like)
3. `impl Bar for Foo` → `Edge { Implements }` (Foo implements Bar)
4. `trait Foo: Bar + Baz` → `Edge { Inherits }` for each supertrait
5. `fn foo()` → `Node { Function { is_async } }`
6. `pub fn` / `pub(crate) fn` → mark node as public

Graceful handling:
- Macro bodies (`macro_rules!`, `#[proc_macro]`) — skip their contents entirely
- Lifetimes (`'a`) — strip from all labels before creating nodes
- Generic parameters — strip angle brackets from type names in labels
- Workspace crates — treat each `Cargo.toml` crate as a sub-graph with a root `Module` node

Tests: parse a Rust snippet with structs, traits, impl blocks, and functions.
""".strip()
    },

    # ── Phase 6: Java/Kotlin Parser ──────────────────────────

    15: {
        "name": "15 — Java Parser",
        "phase": "Phase 6: Java/Kotlin",
        "prompt": """
## Objective
Implement the Java language parser using Tree-sitter.

## Files to Create
- `codeviz-java/Cargo.toml`
- `codeviz-java/src/lib.rs`
- `codeviz-java/src/parser.rs`

## Requirements
Extract into `CodeGraph`:
1. `import com.company.Module` → `Edge { Imports }`
2. `class Foo extends Bar` → `Edge { Inherits }`
3. `class Foo implements IBar, IBaz` → one `Edge { Implements }` per interface
4. `interface IFoo` → `Node { Interface }`
5. `public void method()` → `Node { Function }`
6. Annotations (`@Override`, `@Autowired`) — store as metadata on the node, not as edges

Graceful handling:
- Anonymous classes — skip
- Lambda expressions — skip
- Wildcard imports (`import com.company.*`) — create a single `Imports` edge to the package node

Tests: parse a Java snippet with imports, class hierarchy, and method definitions.
""".strip()
    },

    # ── Phase 7: V1.0 Config & Polish ────────────────────────

    16: {
        "name": "16 — codeviz.toml Config",
        "phase": "Phase 7: V1.0",
        "prompt": """
## Objective
Add `codeviz.toml` configuration file support to `codeviz-core` and the CLI.

## Files to Create/Modify
- `codeviz-core/src/config.rs`
- `codeviz-cli/src/main.rs` (load config, merge with CLI flags)
- `codeviz.toml.example` (at repo root, committed as a reference)

## Config Schema (implement with `serde` + `toml` crate)
```toml
[graph]
max_depth = 3              # limit edge traversal depth (0 = unlimited)
diagram_type = "module"    # module | call | class
max_nodes = 50             # truncate graph above this node count
include = ["src/**"]       # glob patterns to include
exclude = ["**/tests/**", "**/vendor/**", "**/target/**"]

[languages]
enabled = ["python", "typescript", "go", "rust", "java"]

[output]
sentinel_start = "<!-- CODEVIZ_START -->"
sentinel_end   = "<!-- CODEVIZ_END -->"
```

CLI flag precedence: CLI args > `codeviz.toml` > defaults.
Print a warning if `codeviz.toml` is not found (don't error).
Write unit tests covering parsing logic, default fallbacks, and merge precedence.
""".strip()
    },

    17: {
        "name": "17 — VS Code Extension Stub",
        "phase": "Phase 7: V1.0",
        "prompt": """
## Objective
Create the VS Code extension that renders a live CodeViz diagram in the editor sidebar.

## Files to Create
- `vscode-extension/package.json`
- `vscode-extension/src/extension.ts`
- `vscode-extension/src/panel.ts`
- `vscode-extension/README.md`

## Requirements
1. Activate on: any workspace containing a `codeviz.toml` file.
2. Add a sidebar panel ("CodeViz") showing the current file's module graph.
3. On file save: call `codeviz run --path . --diagram module` as a child process.
4. Parse stdout (Mermaid string), render via [Mermaid.js](https://mermaid.js.org/) in a webview panel.
5. Show a status bar item: "CodeViz: Ready" / "CodeViz: Parsing..." / "CodeViz: Error".
6. Bundle WASM module instead of requiring a local `codeviz` binary (optional, as a feature flag).
7. Setup basic Mocha/Chai tests for the extension activation.
""".strip()
    },
}

# ─────────────────────────────────────────────────────────────
# Batch definitions
# ─────────────────────────────────────────────────────────────

BATCHES = {
    1: {"desc": "Phase 0: Foundation — Workspace, IR, Traits, Renderer, Injector", "tasks": [1, 2, 3, 4, 5]},
    2: {"desc": "Phase 1: Python Parser + CLI + CI",                               "tasks": [6, 7, 8]},
    3: {"desc": "Phase 2: TypeScript Parser + WASM Build",                         "tasks": [9, 10]},
    4: {"desc": "Phase 3: MCP Server + Integration Tests",                         "tasks": [11, 12]},
    5: {"desc": "Phase 4 & 5: Go + Rust Parsers",                                  "tasks": [13, 14]},
    6: {"desc": "Phase 6 & 7: Java Parser + Config + VS Code Extension",           "tasks": [15, 16, 17]},
}

# ─────────────────────────────────────────────────────────────
# API helpers
# ─────────────────────────────────────────────────────────────

def _post_session(full_prompt: str, label: str):
    payload = json.dumps({
        "prompt": full_prompt,
        "sourceContext": {
            "source": REPO_SOURCE,
            "githubRepoContext": {
                "startingBranch": BRANCH
            }
        }
    }).encode()

    req = urllib.request.Request(
        API_URL,
        data=payload,
        headers={"Content-Type": "application/json", "x-goog-api-key": API_KEY},
        method="POST"
    )

    try:
        with urllib.request.urlopen(req) as resp:
            result = json.loads(resp.read())
            session_id = result.get("name", "unknown").split("/")[-1]
            print(f"  ✅ Session: {session_id}  →  https://jules.google.com/session/{session_id}")
            return session_id
    except urllib.error.HTTPError as e:
        body = e.read().decode()
        print(f"  ❌ HTTP {e.code} for [{label}]: {body}")
        return None

def _build_prompt(task: dict) -> str:
    """Merge safety rules + inline/file prompt."""
    file_prompt = _load_prompt(task.get("slug", ""))
    body = file_prompt if file_prompt else task.get("prompt", "")
    return SAFETY_RULES + "\n\n---\n\n" + body

# ─────────────────────────────────────────────────────────────
# Commands
# ─────────────────────────────────────────────────────────────

def submit_task(task_num: int):
    task = TASKS[task_num]
    print(f"🚀 Submitting: [{task_num:>2}] {task['name']}  (branch: {BRANCH})")
    _post_session(_build_prompt(task), task["name"])

def submit_batch(batch_num: int):
    batch = BATCHES[batch_num]
    print(f"\n📦 Submitting Batch {batch_num}: {batch['desc']}")
    print(f"   Tasks: {batch['tasks']}  |  Branch: {BRANCH}\n")
    for t in batch["tasks"]:
        task = TASKS[t]
        print(f"  → [{t:>2}] {task['name']}")
        _post_session(_build_prompt(task), task["name"])

def list_tasks():
    print("\n📋 CodeViz Jules Tasks\n")
    current_phase = None
    for num, task in TASKS.items():
        phase = task["phase"]
        if phase != current_phase:
            print(f"\n  ── {phase} ──")
            current_phase = phase
        print(f"    [{num:>2}] {task['name']}")

    print("\n\n📦 Predefined Parallel Batches:\n")
    for num, batch in BATCHES.items():
        print(f"  Batch {num}: {batch['desc']}")
        print(f"           Tasks: {batch['tasks']}")
    print()

def main():
    args = sys.argv[1:]

    if "--list" in args or not args:
        list_tasks()
    elif "--batch" in args:
        idx = args.index("--batch")
        submit_batch(int(args[idx + 1]))
    elif "--task" in args:
        idx = args.index("--task")
        submit_task(int(args[idx + 1]))
    else:
        print("Usage:")
        print("  python scripts/jules_submit.py --list")
        print("  python scripts/jules_submit.py --task <N>")
        print("  python scripts/jules_submit.py --batch <N> [--branch <branch>]")

if __name__ == "__main__":
    main()
