import re

with open('codeviz-cli/tests/watch_test.rs', 'r') as f:
    content = f.read()

# Wait, how many diagram updates?
# Initial run = 1.
# But `notify` watcher is started AFTER `run_core`. Then `watch_test` creates `main.rs`!
# Ah! In the test:
# fs::write(&file_path, "fn main() {}").unwrap(); // This is BEFORE spawn!
# But then the watcher starts and might trigger for something else? No.
# Then 5 writes:
# for _ in 0..5 { append ... }
# Then wait 1500ms
# Then 1 write of bad file
# Then wait 1500ms

# Why is it updating 9 times?!
