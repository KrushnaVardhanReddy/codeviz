import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

content = content.replace("if let Ok(_) = ctrlc_rx.try_recv() {", "if ctrlc_rx.try_recv().is_ok() {")

replacement_if = """                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ["rs", "py", "ts", "tsx", "go", "java", "kt"].contains(&ext) {
                            should_trigger = true;
                            break;
                        }
                    }"""

new_if = """                    if let Some(ext) = path.extension().and_then(|e| e.to_str())
                        && ["rs", "py", "ts", "tsx", "go", "java", "kt"].contains(&ext)
                    {
                        should_trigger = true;
                        break;
                    }"""

content = content.replace(replacement_if, new_if)

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
