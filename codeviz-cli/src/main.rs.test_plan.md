None of the parsers return `ParseError` on syntax error because Tree-Sitter's design is resilient. It returns an AST with `ERROR` nodes.

Wait. The review says:
"The correct approach would have been to simply write a file with actual invalid syntax (e.g., `fn invalid() { {`) during the test to trigger a real parser error."

Wait! Is there an `ERROR` node check in the parsers?
Let's check if the Rust parser checks for `ERROR` nodes!
