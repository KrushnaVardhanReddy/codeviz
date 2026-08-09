pub mod config;
pub mod engine;
pub mod grammar_map;

pub use config::LangConfig;
pub use engine::GenericParser;
use codeviz_core::parser::LanguageRegistry;

/// Loads all pre-defined language parsers from embedded TOML files.
pub fn all_parsers() -> Vec<GenericParser> {
    let toml_files = vec![
        include_str!("../languages/ruby.toml"),
        include_str!("../languages/swift.toml"),
        include_str!("../languages/csharp.toml"),
        include_str!("../languages/php.toml"),
        include_str!("../languages/dart.toml"),
        include_str!("../languages/lua.toml"),
    ];

    let mut parsers = Vec::new();
    for toml_str in toml_files {
        if let Ok(config) = toml::from_str::<LangConfig>(toml_str) {
            parsers.push(GenericParser::new(config));
        }
    }
    parsers
}

/// Registers all available generic language parsers into the given registry.
pub fn register_all(registry: &mut LanguageRegistry) {
    for parser in all_parsers() {
        registry.register(Box::new(parser));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codeviz_core::parser::LanguageParser;

    #[test]
    fn test_ruby_parser() {
        let toml_str = include_str!("../languages/ruby.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            class MyClass
                def my_func
                end
            end
        ";
        let graph = parser.parse(code, "test.rb").unwrap();

        assert_eq!(graph.meta.language, "ruby");
        assert!(graph.nodes.iter().any(|n| n.label == "MyClass"));
        assert!(graph.nodes.iter().any(|n| n.label == "my_func"));
    }

    #[test]
    fn test_swift_parser() {
        let toml_str = include_str!("../languages/swift.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            class MyClass {
                func myFunc() {}
            }
        ";
        let graph = parser.parse(code, "test.swift").unwrap();

        assert_eq!(graph.meta.language, "swift");
        assert!(graph.nodes.iter().any(|n| n.label == "MyClass"));
        assert!(graph.nodes.iter().any(|n| n.label == "myFunc"));
    }

    #[test]
    fn test_csharp_parser() {
        let toml_str = include_str!("../languages/csharp.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            class MyClass {
                void MyFunc() {}
            }
        ";
        let graph = parser.parse(code, "test.cs").unwrap();

        assert_eq!(graph.meta.language, "csharp");
        assert!(graph.nodes.iter().any(|n| n.label == "MyClass"));
        assert!(graph.nodes.iter().any(|n| n.label == "MyFunc"));
    }

    #[test]
    fn test_php_parser() {
        let toml_str = include_str!("../languages/php.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            <?php
            class MyClass {
                public function myFunc() {}
            }
        ";
        let graph = parser.parse(code, "test.php").unwrap();

        assert_eq!(graph.meta.language, "php");
        assert!(graph.nodes.iter().any(|n| n.label == "MyClass"));
        assert!(graph.nodes.iter().any(|n| n.label == "myFunc"));
    }

    #[test]
    fn test_dart_parser() {
        let toml_str = include_str!("../languages/dart.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            class MyClass {
                void myFunc() {}
            }
        ";
        let graph = parser.parse(code, "test.dart").unwrap();

        assert_eq!(graph.meta.language, "dart");
        assert!(graph.nodes.iter().any(|n| n.label == "MyClass"));
        assert!(graph.nodes.iter().any(|n| n.label == "myFunc"));
    }

    #[test]
    fn test_lua_parser() {
        let toml_str = include_str!("../languages/lua.toml");
        let config: LangConfig = toml::from_str(toml_str).unwrap();
        let parser = GenericParser::new(config);

        let code = "
            function myFunc()
            end
        ";
        let graph = parser.parse(code, "test.lua").unwrap();

        assert_eq!(graph.meta.language, "lua");
        assert!(graph.nodes.iter().any(|n| n.label == "myFunc"));
    }
}
