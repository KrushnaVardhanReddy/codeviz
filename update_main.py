import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Add watch to args matching
watch_block = """
    if args.len() > 1 && args[1] == "watch" {
        let run_args = parse_run_args(&args[2..])?;
        return crate::watch::run_watch(&run_args);
    }
"""

start_idx = content.find('    if args.len() > 1 && (args[1] == "run" || args[1] == "check") {')
new_content = content[:start_idx] + watch_block + "\n" + content[start_idx:]

# Update help output
help_idx = new_content.find('println!("  run          Parses source code and injects an updated diagram into a markdown file.");')
help_str = 'println!("  run          Parses source code and injects an updated diagram into a markdown file.");\n    println!("  watch        Watches source directory and automatically re-runs parse on file save.");'
new_content = new_content.replace('println!("  run          Parses source code and injects an updated diagram into a markdown file.");', help_str)

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(new_content)
