import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

backdoor = """                if source.contains("TRIGGER_PARSE_ERROR_FOR_TEST") {
                    parse_errors.push(codeviz_core::parser::ParseError {
                        message: "Simulated parse error".to_string(),
                        file_path: file.to_string_lossy().to_string(),
                        line: Some(14),
                    });
                    continue;
                }

"""

content = content.replace(backdoor, "")

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
