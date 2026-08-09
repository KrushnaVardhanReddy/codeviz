pub mod dot;
pub mod json;
pub mod mermaid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Mermaid,
    Json,
    Dot,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mermaid" => Ok(OutputFormat::Mermaid),
            "json" => Ok(OutputFormat::Json),
            "dot" => Ok(OutputFormat::Dot),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}
