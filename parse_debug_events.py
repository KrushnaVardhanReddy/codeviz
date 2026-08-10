import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Let's see if run_core writes a file that causes a watch loop.
# It writes to `out.md` and `.codeviz_cache/meta.json`.
# If `run_core` takes 1 second to write, `notify` will emit events.
# But `out.md` and `.json` are NOT in the supported extension list!
# Let's check `watch` logic:
