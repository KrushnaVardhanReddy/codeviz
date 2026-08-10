TASK: T60 — Cross-Platform Path Normalization (Windows Compatibility)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Ensure the CodeViz Rust CLI produces consistent graph output on Windows by
normalizing all internal file paths to use forward slashes (`/`). The
`Node::id` and `Node::file_path` fields must always use `/`, regardless of
the host OS. Additionally, add a Windows CI runner to verify this fix.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/qa_blind_spots.md (Section 1: Cross-Platform Path Normalization)

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS (READ ONLY — do not modify unless specified)
═══════════════════════════════════════════════════════════════

THE PROBLEM:
  On Windows, Rust's `std::path::Path` uses backslash (`\`) as the separator.
  When parsers build Node IDs like `"{file_path}::{symbol}"`, any
  backslash-separated path will break downstream consumers (the web UI, the
  MCP server, JSON export, etc.) that expect forward-slash-separated paths.

CORE IR (READ ONLY — understand the struct, do not change the struct definition):
  codeviz-core/src/graph.rs (also re-exported from codeviz-core/src/ir.rs)
  pub struct Node {
      pub id: String,          // MUST always use `/` separators
      pub file_path: String,   // MUST always use `/` separators
      ...
  }

WHERE PATHS ARE BUILT — the fix must be applied in these files:
  codeviz-core/src/graph.rs     — helper functions that build Node { id, file_path }
  codeviz-core/src/parser.rs    — LanguageRegistry::parse_file and parse_directory
                                   iterate over directory entries; paths must be normalized
                                   before they are passed into parsers.
  Language parsers (look inside each for where they construct Node { id, file_path }):
    codeviz-python/src/parser.rs
    codeviz-typescript/src/parser.rs
    codeviz-rust/src/parser.rs
    codeviz-go/src/parser.rs
    codeviz-java/src/parser.rs
    codeviz-kotlin/src/parser.rs
    codeviz-langs/src/lib.rs   (covers: Lua, PHP, Ruby, Swift, C#, Dart)

EXISTING CI WORKFLOW (MODIFY to add Windows E2E runner):
  .github/workflows/ci.yml
  - The `test` job already runs `cargo test --all` on ubuntu-latest, macos-latest, and windows-latest.
  - There is currently NO Playwright E2E job. You will add one.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. ADD: codeviz-core/src/path_utils.rs (NEW FILE)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Create a small utility module with a single public function:

  /// Normalize a file path to always use forward slashes.
  /// This is necessary for cross-platform compatibility on Windows.
  pub fn normalize_path(path: &str) -> String {
      path.replace('\\', "/")
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_forward_slash_unchanged() {
          assert_eq!(normalize_path("src/main.rs"), "src/main.rs");
      }

      #[test]
      fn test_backslash_converted() {
          assert_eq!(normalize_path("src\\main.rs"), "src/main.rs");
      }

      #[test]
      fn test_mixed_slashes() {
          assert_eq!(normalize_path("src\\lib\\utils.rs"), "src/lib/utils.rs");
      }
  }

Then register it in codeviz-core/src/lib.rs:
  pub mod path_utils;


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: All language parsers — normalize Node::id and Node::file_path
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

In EVERY parser that constructs a `Node { id, file_path, ... }`, wrap the path
string with `codeviz_core::path_utils::normalize_path(...)` before using it.

The typical pattern to find and fix looks like this:

  BEFORE:
    Node {
        id: format!("{}::{}", file_path, name),
        file_path: file_path.to_string(),
        ...
    }

  AFTER:
    use codeviz_core::path_utils::normalize_path;
    Node {
        id: format!("{}::{}", normalize_path(file_path), name),
        file_path: normalize_path(file_path),
        ...
    }

Apply this fix in these files:
  - codeviz-python/src/parser.rs
  - codeviz-typescript/src/parser.rs
  - codeviz-rust/src/parser.rs
  - codeviz-go/src/parser.rs
  - codeviz-java/src/parser.rs
  - codeviz-kotlin/src/parser.rs
  - codeviz-langs/src/lib.rs

Also apply the fix to `codeviz-core/src/parser.rs` where `parse_directory`
iterates over files and constructs the relative `file_path` string that is
passed to each parser. Normalize it before passing it down.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. ADD: Unit test for normalize_path in parser integration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

In codeviz-core/src/parser.rs (or a new test file), add:

  #[test]
  fn test_node_ids_use_forward_slashes() {
      // Simulate a Windows-style path being passed to the parser
      let file_path = "src\\lib\\utils.py";
      let normalized = codeviz_core::path_utils::normalize_path(file_path);
      assert!(!normalized.contains('\\'), "Node paths must not contain backslashes");
      assert_eq!(normalized, "src/lib/utils.py");
  }


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
4. MODIFY: .github/workflows/ci.yml — Add Playwright E2E job with Windows runner
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Add a new job `e2e` to ci.yml AFTER the existing `lint` job:

  e2e:
    name: Playwright E2E on ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    defaults:
      run:
        working-directory: codeviz-web
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install dependencies
        run: npm ci
      - name: Install Playwright browsers
        run: npx playwright install --with-deps
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build Rust CLI
        run: cargo build --release
        working-directory: .
      - name: Run E2E tests
        run: npm run test:e2e

DO NOT remove or modify the existing `test`, `lint`, `wasm-build`, or
`wasm-size` jobs. Only ADD the new `e2e` job.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
5. VERIFY: Run `cargo test --all` and confirm all tests pass
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

After all changes, run:
  cargo test --all

All existing tests (35 in codeviz-core, all language parsers) must still pass.
The 3 new tests in path_utils must also pass.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════

1. The normalize_path function must be the SINGLE source of truth for path
   normalization. Do not inline `replace('\\', "/")` in individual parsers.
   Always call the shared util.

2. Do NOT change the Node struct definition in graph.rs or ir.rs. The fix
   is purely at the call sites where Node { id, file_path } is constructed.

3. Do NOT break any existing parser tests. All existing tests that pass
   forward-slash paths should continue to pass unchanged.

4. Do NOT modify any web UI files (codeviz-web/).

5. Do NOT modify any spec files (docs/specs/).

═══════════════════════════════════════════════════════════════
FILES LIST
═══════════════════════════════════════════════════════════════

FILES TO CREATE:
  codeviz-core/src/path_utils.rs

FILES TO MODIFY:
  codeviz-core/src/lib.rs               (register path_utils module)
  codeviz-core/src/parser.rs            (normalize path before passing to parsers)
  codeviz-python/src/parser.rs
  codeviz-typescript/src/parser.rs
  codeviz-rust/src/parser.rs
  codeviz-go/src/parser.rs
  codeviz-java/src/parser.rs
  codeviz-kotlin/src/parser.rs
  codeviz-langs/src/lib.rs
  .github/workflows/ci.yml              (add e2e job with windows-latest runner)

FILES NOT TO TOUCH (READ-ONLY):
  codeviz-core/src/ir.rs               (IR struct definitions)
  codeviz-core/src/graph.rs            (CodeGraph struct — do not change struct)
  codeviz-web/                          (all web UI files)
  codeviz-vscode/                       (VS Code extension)
  docs/specs/                           (all spec files)

Commit: "jules: T60 — Cross-platform path normalization with forward-slash enforcement"
Target branch: feat-t60-windows-paths
