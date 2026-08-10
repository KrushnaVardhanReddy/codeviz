with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# wait, we didn't specify run_core correctly in module or watch module didn't see it because it's in main.rs and main.rs is the root.
# Oh, main.rs is not a lib. We shouldn't use `crate::run_core` if `run_core` is in main.rs! Wait, we can use `crate::run_core`.
