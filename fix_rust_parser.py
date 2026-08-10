import re

with open('codeviz-rust/src/parser.rs', 'r') as f:
    content = f.read()

# Let's add a check for has_error() in codeviz-rust parser to actually return ParseError on syntax error
# tree.root_node().has_error() is available in tree-sitter.

replacement = """        let tree = match parser.parse(source, None) {
            Some(tree) => tree,
            None => {
                return Err(ParseError {
                    message: "Failed to parse source code".to_string(),
                    file_path: file_path.to_string(),
                    line: None,
                });
            }
        };

        if tree.root_node().has_error() {
            return Err(ParseError {
                message: "Syntax error".to_string(),
                file_path: file_path.to_string(),
                line: Some(tree.root_node().start_position().row as u32 + 1),
            });
        }
"""

content = content.replace("""        let tree = match parser.parse(source, None) {
            Some(tree) => tree,
            None => {
                return Err(ParseError {
                    message: "Failed to parse source code".to_string(),
                    file_path: file_path.to_string(),
                    line: None,
                });
            }
        };""", replacement)

with open('codeviz-rust/src/parser.rs', 'w') as f:
    f.write(content)
