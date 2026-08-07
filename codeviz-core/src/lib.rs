pub mod inject;
pub use inject::{inject_mermaid, InjectError};

pub mod graph;
pub mod ir;
pub mod parser;
pub mod render;

pub use ir::*;
pub use parser::*;

/// A core struct for CodeViz representing a dummy node to pass initial tests.
pub struct DummyNode {
    /// The name of the dummy node.
    pub name: String,
}

impl DummyNode {
    /// Creates a new `DummyNode` with the specified name.
    /// Returns a Result to satisfy mandatory rules without using unwrap.
    pub fn new(name: String) -> Result<Self, String> {
        if name.is_empty() {
            Err("Name cannot be empty".to_string())
        } else {
            Ok(Self { name })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_node_creation() {
        let node = DummyNode::new("Test".to_string()).expect("Should create successfully in test");
        assert_eq!(node.name, "Test");
    }

    #[test]
    fn test_dummy_node_empty_name() {
        let node = DummyNode::new("".to_string());
        assert!(node.is_err());
    }
}
