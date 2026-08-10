import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# wait, `run_core` was defined INSIDE `run_cli` ! Let's check!
