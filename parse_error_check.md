Ah, in the current `run` command:
```rust
                if let Ok(source) = std::fs::read_to_string(&file) {
                    match registry.parse_file(&file.to_string_lossy(), &source) {
                        Ok(graph) => {
                            ...
                        }
                        Err(e) => {
                            // Currently ignored in if-let!
                        }
                    }
                }
```
If we want to capture the error in watch mode we might want to collect errors and return them from `execute_run`. Or we can just print the error and continue, but the spec says "On error: print `[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`".
So we should probably modify the parse loop to catch errors and maybe return a list of them, or the first one?
Wait, if it returns an error, the diagram isn't updated?
Let's see: `On error: print ... Do NOT exit.` "Continue watching after errors."
Wait, if there's an error parsing one file, do we still update the diagram with the rest of the files, or fail the whole run? Usually in watch mode, if there's a syntax error, we don't update the output.
But `execute_run` just returns early if there's an error?
Let's collect errors and if there's any, return `Err(e)` where `e` has the formatted string.
