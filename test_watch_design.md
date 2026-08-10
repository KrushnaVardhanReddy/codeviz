## Plan for watch mode

1. Extract the `run` logic from `run_cli` into a `fn execute_run(run_args: &RunArgs, is_check: bool) -> Result<bool, String>` so that we can call it from both `run`/`check` command handling and `watch` command handling. Wait, we should just extract the inner logic. Or maybe wrap the `args[1] == "run"` block's interior into a separate function.

Looking at `run_cli`, the `run` and `check` logic is huge.
Lines 654 to 865 is `if args.len() > 1 && (args[1] == "run" || args[1] == "check") { ... }`.
I should extract this into:
```rust
fn execute_run(run_args: &RunArgs, is_check: bool) -> Result<bool, String> {
   ...
}
```

2. Add a `watch` subcommand parsing block in `run_cli`.
```rust
    if args.len() > 1 && args[1] == "watch" {
        let run_args = parse_run_args(&args[2..])?;
        return run_watch(&run_args);
    }
```

3. Implement `run_watch`:
```rust
use notify::{Watcher, RecursiveMode, EventKind, event::ModifyKind};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn run_watch(run_args: &RunArgs) -> Result<bool, String> {
    // Initial run
    let _ = execute_run(run_args, false);

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)
        .map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher.watch(Path::new(&run_args.path), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch path {}: {}", run_args.path, e))?;

    println!("Watching {} for changes...", run_args.path);

    let mut last_event_time = std::time::Instant::now();
    let debounce_duration = Duration::from_millis(300);
    let mut pending_update = false;

    // ctrlc handler
    let (ctrlc_tx, ctrlc_rx) = channel();
    ctrlc::set_handler(move || {
        let _ = ctrlc_tx.send(());
    }).map_err(|e| format!("Error setting Ctrl-C handler: {}", e))?;

    loop {
        if let Ok(_) = ctrlc_rx.try_recv() {
            println!("Exiting watch mode cleanly.");
            return Ok(true);
        }

        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(Ok(event)) => {
                // Filter events based on extension and kind
                let mut should_trigger = false;
                for path in event.paths {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ["rs", "py", "ts", "go", "java", "kt"].contains(&ext) {
                            should_trigger = true;
                            break;
                        }
                    }
                }

                if should_trigger {
                    pending_update = true;
                    last_event_time = std::time::Instant::now();
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if pending_update && last_event_time.elapsed() >= debounce_duration {
                    pending_update = false;
                    let now = chrono::Local::now().format("%H:%M:%S").to_string();
                    match execute_run(run_args, false) {
                        Ok(_) => {
                            // execute_run should print on success, but maybe we modify execute_run to return the stats so we can print the required message:
                            // `[14:32:01] ✅ Diagram updated — 12 nodes, 8 edges`
                        }
                        Err(e) => {
                            // `[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`
                            println!("[{}] ❌ Parse error: {}", now, e);
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(true)
}
```

Wait, `execute_run` prints:
```
Successfully parsed {} files, generated diagram with {} nodes and {} edges. Output: {}
```
We can modify `execute_run` to return `(usize, usize)` i.e., `(node_count, edge_count)` on success instead of `bool`. Or return `RunResult` struct.

Let's inspect how the prompt wants output format:
`[14:32:01] ✅ Diagram updated — 12 nodes, 8 edges`
`[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`

If we return `(node_count, edge_count)` we can easily print this.

Let's inspect error printing. Does `registry.parse_file` throw error? Yes it returns Result.
Actually `execute_run` just suppresses individual file parse errors and continues currently? Let's check.
