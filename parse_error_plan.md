Okay, we can print the parse error. `run_cli` is currently returning `Result<bool, String>`.
I will refactor the inner logic of `run` / `check` block out, and then we will have `watch` command use it.

```rust
// In `main.rs`
// First, add `watch` to help message:
    println!("  run          Parses source code and...");
    println!("  watch        Watches source directory and automatically re-runs parse on file save.");

// Refactor the main processing:
fn execute_run(run_args: &RunArgs, is_check: bool) -> Result<RunResult, Vec<codeviz_core::parser::ParseError>> {
    // same implementation, but we gather ParseErrors instead of ignoring them
    // wait, if we gather them, do we stop?
    // "A parse error prints the error but the watcher continues running."
    // Let's modify the file loop to collect errors.

    // ...
```

Wait, `codeviz run` might also want to report these errors?
In `run_cli` currently:
```rust
                if let Ok(source) = std::fs::read_to_string(&file)
                    && let Ok(graph) = registry.parse_file(&file.to_string_lossy(), &source)
                {
                    ...
                }
```
It currently ignores all IO errors and ParseErrors!
So I can just change it to:
```rust
                if let Ok(source) = std::fs::read_to_string(&file) {
                    match registry.parse_file(&file.to_string_lossy(), &source) {
                        Ok(graph) => {
                            // ...
                        }
                        Err(e) => {
                            errors.push(e);
                        }
                    }
                }
```
Then `execute_run` can return `Result<RunResult, Vec<ParseError>>`.
Actually, if there are errors, should `execute_run` still update the diagram?
Usually yes! We update it with whatever successfully parsed.
Wait, spec: "On error: print `[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`".
So if there are errors, we print them.

Let's look at the exact acceptance criteria:
- Saving a `.py` file triggers a diagram update within 400ms.
- Saving a `.txt` file does NOT trigger an update.
- A parse error prints the error but the watcher continues running.
- Ctrl+C exits with code 0.
