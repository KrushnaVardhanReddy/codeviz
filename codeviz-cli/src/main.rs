use codeviz_core::DummyNode;
use std::env;

/// Prints the help message for the CLI.
pub fn print_help() {
    println!("codeviz --help");
    println!("Usage: codeviz [OPTIONS]");
    println!("Options:");
    println!("  --help  Print this help message");
}

/// Main entry point for the CLI.
/// Returns a Result to satisfy mandatory rules without using unwrap.
pub fn run_cli(args: Vec<String>) -> Result<(), String> {
    if args.iter().any(|arg| arg == "--help") {
        print_help();
        return Ok(());
    }

    // Dummy usage of core to prove dependency works
    let _node = DummyNode::new("cli-node".to_string())?;

    println!("CodeViz CLI (No options provided, try --help)");
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    run_cli(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_flag() {
        let args = vec!["codeviz".to_string(), "--help".to_string()];
        let result = run_cli(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_flags() {
        let args = vec!["codeviz".to_string()];
        let result = run_cli(args);
        assert!(result.is_ok());
    }
}
