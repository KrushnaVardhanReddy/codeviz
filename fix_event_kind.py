import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# We need to filter event.kind
# It should only trigger if `event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove()`
# Wait, `notify` event `kind` has `EventKind`.
# If we do:
# `if !matches!(event.kind, notify::EventKind::Access(_)) { ... }`

replacement = """            Ok(Ok(event)) => {
                if matches!(event.kind, notify::EventKind::Access(_)) {
                    continue;
                }

                let mut should_trigger = false;"""

content = content.replace("            Ok(Ok(event)) => {\n                let mut should_trigger = false;", replacement)
content = content.replace('println!("Event triggered by {:?}", event.paths);', '')

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
