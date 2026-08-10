import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Since watch is included in main via `pub mod watch;`, we can use `super::run_core` or `crate::run_core`.
# But wait, in bin targets `crate::` refers to `main.rs`. Why is it missing?
# Maybe run_core is inside a `mod`? No.
# Oh, main.rs is compiled as `codeviz-cli`, not a library!
# Let's check `codeviz-cli/src/main.rs` to see if `RunResult` and `run_core` are `pub`.

with open('codeviz-cli/src/watch.rs', 'r') as f:
    watch_content = f.read()

watch_content = watch_content.replace('use crate::{RunArgs, run_core, RunResult};', 'use super::{RunArgs, run_core, RunResult};')

with open('codeviz-cli/src/watch.rs', 'w') as f:
    f.write(watch_content)
