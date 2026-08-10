We can refactor `run_cli` to extract `run_core(args: &RunArgs, is_check: bool, is_watch: bool) -> Result<(usize, usize), String>`.
If `is_watch` is true, we suppress the default `println!` statements and instead print the timestamped ones. Or we can just have `run_core` return `Ok((nodes, edges))` and `run_cli` does the printing.

Wait, if there's a parse error in a file, we should return that error in watch mode?
Currently, if `registry.parse_file` fails:
```rust
                if let Ok(source) = std::fs::read_to_string(&file) {
                    match registry.parse_file(&file.to_string_lossy(), &source) {
                        Ok(graph) => { ... }
                        Err(e) => {
                            if is_watch {
                                return Err(format!("Parse error in {}: {}", file.display(), e));
                            }
                        }
                    }
                }
```
Wait, the error message in the spec:
`[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`
But `parse_file` returns a `String`. I don't think it necessarily has `:14` inside unless it does.
Let's see what `e` is. `e` is the string returned by `parse_file`.

Wait, the spec mentions:
"Saving a `.py` file triggers a diagram update within 400ms."
"Saving a `.txt` file does NOT trigger an update."
"A parse error prints the error but the watcher continues running."
"Ctrl+C exits with code 0."

Let's look at `codeviz-python/src/lib.rs` to see what `parse_file` returns on error.
