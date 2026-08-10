with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# run_core was defined inside run_cli!
start_idx = content.find('pub struct RunResult {')
# find end of run_cli which was before `// Default error for unknown subcommands`

end_idx = content.find('    // Default error for unknown subcommands')

if end_idx != -1 and start_idx != -1:
    extracted = content[start_idx:end_idx]

    new_content = content[:start_idx] + "\n" + content[end_idx:] + "\n" + extracted

    with open('codeviz-cli/src/main.rs', 'w') as f:
        f.write(new_content)
else:
    print("Could not find blocks")
