import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# `pub mod watch;` needs to be after we declare the structs it uses? No, order in Rust doesn't matter for mods in same crate.
# Wait! Let's just put watch.rs contents directly in main.rs so we don't have to deal with bin module weirdness.

with open('codeviz-cli/src/watch.rs', 'r') as f:
    watch_content = f.read()

watch_content = watch_content.replace('use super::{RunArgs, run_core, RunResult};', '')

content = content.replace('pub mod watch;\n', '')
content = content + "\n\n" + watch_content

# We need to replace crate::watch::run_watch with run_watch
content = content.replace('crate::watch::run_watch', 'run_watch')

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
