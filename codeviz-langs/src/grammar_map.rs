use codeviz_core::parser::ParseError;

/// Returns the `tree_sitter::Language` for a given grammar name.
/// This acts as a static registry mapping grammar string identifiers to their parsed grammars.
pub fn get_language(name: &str) -> Result<tree_sitter::Language, ParseError> {
    let lang = match name {
        "tree-sitter-ruby" => tree_sitter_ruby::language(),
        "tree-sitter-swift" => tree_sitter_swift::language(),
        "tree-sitter-c-sharp" => tree_sitter_c_sharp::language(),
        "tree-sitter-php" => tree_sitter_php::language_php(),
        "tree-sitter-dart" => tree_sitter_dart::language(),
        "tree-sitter-lua" => tree_sitter_lua::language(),
        _ => {
            return Err(ParseError {
                message: format!("Unknown grammar: {}", name),
                file_path: String::new(),
                line: None,
            })
        }
    };
    Ok(lang)
}
