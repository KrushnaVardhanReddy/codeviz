use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Config {
    pub graph: GraphConfig,
    pub languages: LanguagesConfig,
    pub output: OutputConfig,
    pub cache: CacheConfig,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct GraphConfig {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub diagram_type: String,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            max_depth: 0,
            max_nodes: 50,
            diagram_type: "module".to_string(),
            include: vec!["**".to_string()],
            exclude: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/vendor/**".to_string(),
            ],
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct LanguagesConfig {
    pub enabled: Vec<String>,
}

impl Default for LanguagesConfig {
    fn default() -> Self {
        Self {
            enabled: vec![
                "python".to_string(),
                "typescript".to_string(),
                "go".to_string(),
                "rust".to_string(),
                "java".to_string(),
                "kotlin".to_string(),
            ],
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct OutputConfig {
    pub sentinel_start: String,
    pub sentinel_end: String,
    pub targets: Vec<String>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            sentinel_start: "<!-- CODEVIZ_START -->".to_string(),
            sentinel_end: "<!-- CODEVIZ_END -->".to_string(),
            targets: vec!["README.md".to_string()],
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(default)]
pub struct CacheConfig {
    pub enabled: bool,
    pub dir: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dir: ".codeviz_cache".to_string(),
        }
    }
}

impl Config {
    /// Loads the configuration by searching for `codeviz.toml` walking upwards from `start_dir`.
    /// Returns the default config if no file is found.
    pub fn load_from_dir(start_dir: &Path) -> Result<Self, String> {
        let mut current_dir = Some(start_dir.to_path_buf());

        while let Some(dir) = current_dir {
            let config_path = dir.join("codeviz.toml");
            if config_path.is_file() {
                return Self::load_from_file(&config_path);
            }
            current_dir = dir.parent().map(|p| p.to_path_buf());
        }

        Ok(Config::default())
    }

    /// Loads the configuration directly from a file.
    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        if !path.is_file() {
            return Ok(Config::default());
        }
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&content)
            .map_err(|e| format!("Invalid TOML syntax in {}: {}", path.display(), e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_parse_complete_config() {
        let toml_str = r#"
            [graph]
            max_depth = 5
            max_nodes = 100
            diagram_type = "call"
            include = ["src/**"]
            exclude = ["test/**"]

            [languages]
            enabled = ["rust", "go"]

            [output]
            sentinel_start = "<!-- START -->"
            sentinel_end = "<!-- END -->"
            targets = ["DOCS.md"]

            [cache]
            enabled = false
            dir = ".cache"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.graph.max_depth, 5);
        assert_eq!(config.graph.max_nodes, 100);
        assert_eq!(config.graph.diagram_type, "call");
        assert_eq!(config.graph.include, vec!["src/**"]);
        assert_eq!(config.graph.exclude, vec!["test/**"]);

        assert_eq!(config.languages.enabled, vec!["rust", "go"]);

        assert_eq!(config.output.sentinel_start, "<!-- START -->");
        assert_eq!(config.output.sentinel_end, "<!-- END -->");
        assert_eq!(config.output.targets, vec!["DOCS.md"]);

        assert_eq!(config.cache.enabled, false);
        assert_eq!(config.cache.dir, ".cache");
    }

    #[test]
    fn test_parse_partial_config() {
        let toml_str = r#"
            [graph]
            max_depth = 2
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();

        assert_eq!(config.graph.max_depth, 2);
        assert_eq!(config.graph.max_nodes, 50); // Default
        assert_eq!(config.graph.diagram_type, "module"); // Default

        assert_eq!(config.cache.enabled, true); // Default
        assert_eq!(config.cache.dir, ".codeviz_cache"); // Default
    }

    #[test]
    fn test_load_from_dir_discovery() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let config_path = root.join("codeviz.toml");
        fs::write(&config_path, "cache.enabled = false\n").unwrap();

        let sub_dir = root.join("src").join("module");
        fs::create_dir_all(&sub_dir).unwrap();

        let config = Config::load_from_dir(&sub_dir).unwrap();
        assert_eq!(config.cache.enabled, false);
    }
}
