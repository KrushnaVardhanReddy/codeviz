use serde::{Deserialize, Serialize};

/// Represents the top-level configuration in a language TOML file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LangConfig {
    /// The language metadata.
    pub language: LanguageMeta,
    /// The tree-sitter query configurations.
    pub queries: QueryConfig,
}

/// Metadata about the language being defined.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageMeta {
    /// The readable name of the language, e.g., "ruby".
    pub name: String,
    /// The file extensions associated with this language, e.g., ["rb"].
    pub extensions: Vec<String>,
    /// The grammar to use, e.g., "tree-sitter-ruby".
    pub grammar: String,
}

/// A set of tree-sitter queries used to extract code architecture information.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryConfig {
    /// Query for extracting functions.
    pub functions: Option<String>,
    /// Query for extracting async functions.
    pub async_functions: Option<String>,
    /// Query for extracting classes.
    pub classes: Option<String>,
    /// Query for extracting interfaces.
    pub interfaces: Option<String>,
    /// Query for extracting module imports.
    pub imports: Option<String>,
    /// Query for extracting class inheritance.
    pub inheritance: Option<String>,
    /// Query for extracting public exports.
    pub exports: Option<String>,
}
