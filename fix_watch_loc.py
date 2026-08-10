import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# We need to move the run_watch function ABOVE the `mod tests` block.
test_idx = content.find('#[cfg(test)]\nmod tests {')
if test_idx == -1:
    print("Could not find tests block")
else:
    watch_idx = content.find('use std::sync::mpsc::channel;', test_idx)
    watch_code = content[watch_idx:]

    new_content = content[:test_idx] + watch_code + "\n\n" + content[test_idx:watch_idx]
    with open('codeviz-cli/src/main.rs', 'w') as f:
        f.write(new_content)
