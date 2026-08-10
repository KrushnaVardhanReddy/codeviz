1. **Add `notify` crate to `codeviz-cli` Cargo.toml**.
2. **Add `chrono` crate to `codeviz-cli` Cargo.toml** for timestamped logging (already done).
3. **Refactor `run_cli`**:
   - Extract the parsing and generation logic from `run`/`check` block into a standalone function: `fn run_core(run_args: &RunArgs, is_check: bool) -> Result<(usize, usize, Vec<codeviz_core::parser::ParseError>), String>`
   - `run_core` will return the number of nodes, number of edges, and a list of `ParseError`s encountered, or a string on hard failure.
   - Update `run` and `check` to call this function and print outputs/errors as they did before.
4. **Implement `watch` command**:
   - Parse `RunArgs` from `--path` and `--output` flags using `parse_run_args`.
   - Setup `notify` watcher on the specified path, tracking `Modify`, `Create`, `Remove` events.
   - Buffer events and apply a `300ms` debounce logic.
   - On event trigger:
     - Run `run_core`.
     - Print success log using `chrono` for timestamp: `[HH:MM:SS] ✅ Diagram updated — X nodes, Y edges`.
     - If `run_core` returns `ParseError`s, iterate and print them: `[HH:MM:SS] ❌ Parse error in <file>:<line> — <message>`. (Handling `line` correctly if it's `Some`).
   - Allow continuous watching (loop) despite errors.
   - Setup a `ctrlc` handler using a simple channel to gracefully break the loop and return `Ok(true)` (which translates to exit code 0).
5. **Add tests**:
   - Test debounce logic directly (mocking events).
   - Test parse errors during watch don't exit.
   - Wait, `watch` loops infinitely. The spec requires us to test `watch` with unit tests. We can extract the "debounce and run" logic to a testable pure function, or test `watch` with a mock watcher or run in a thread. Since we can't easily mock `notify::Watcher`, we can spawn the command in a child process or thread, write a file, observe stdout, and verify it behaves correctly. We will use `tempfile` and standard IO.
6. **Pre-commit step**:
   - Run `pre_commit_instructions` tool to get tests format and verification steps.
