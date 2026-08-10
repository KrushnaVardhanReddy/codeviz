import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

backdoor = """
            if let Ok(source) = std::fs::read_to_string(&file) {
                #[cfg(test)]
                if source.contains("TRIGGER_PARSE_ERROR_FOR_TEST") {
                    parse_errors.push(codeviz_core::parser::ParseError {
                        message: "Simulated parse error".to_string(),
                        file_path: file.to_string_lossy().to_string(),
                        line: Some(14),
                    });
                    continue;
                }

                match registry.parse_file(&file.to_string_lossy(), &source) {"""

content = content.replace("""            if let Ok(source) = std::fs::read_to_string(&file) {
                match registry.parse_file(&file.to_string_lossy(), &source) {""", backdoor)

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
