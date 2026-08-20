# Manual Entry Points Configuration

## Overview
Certain languages (like Python) do not have a strict, universally named main function that AST parsers can automatically identify as `is_public = true` (an entry point). Without an entry point, features like the Execution Flow Visualization cannot render a proper top-down call tree. 

This spec introduces the ability to manually define entry points in `codeviz.toml`.

## Configuration
The `[graph]` section in `codeviz.toml` will support a new optional array `entry_points`.

```toml
[graph]
entry_points = ["flask.cli::main", "flask.app::Flask.run"]
```

## Behavior
1. During graph processing, if `config.graph.entry_points` is defined, the CLI will iterate through all generated nodes.
2. If a node's ID *ends with* any string specified in `entry_points`, it will be marked as an entry point (its `is_public` property set to `true`). 
   - **Why "ends with"?** Node IDs often contain full paths (e.g., `temp_repos/flask/src/flask/cli.py::main`). Using "ends with" allows users to use shorthand like `flask/cli.py::main` or `cli.py::main` without knowing the absolute workspace path prefix.
3. The renderer will pick up these `is_public` nodes as the roots for the Execution Flow visualization.
