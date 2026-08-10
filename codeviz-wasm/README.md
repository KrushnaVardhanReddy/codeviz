# CodeViz WASM

This package provides a WebAssembly (WASM) module for CodeViz.
It exposes a JS-callable API to render a pre-parsed JSON `CodeGraph` into Mermaid diagrams directly in the browser.

## Constraints
- Max WASM bundle size: < 3 MB
- Execution: zero-upload guarantee, everything happens locally.

## API Usage

The WASM module uses async initialization (wasm-bindgen default).
```javascript
import init, { render_graph } from './codeviz_wasm.js';

await init();

const graphJson = JSON.stringify({
    "nodes": [],
    "edges": [],
    "meta": {
        "language": "python",
        "source_root": "",
        "generated_at": "",
        "node_count": 0,
        "edge_count": 0
    }
});

// Render the pre-parsed CodeGraph JSON into a Mermaid diagram
try {
    const diagram = render_graph(graphJson, "module");
    console.log(diagram);
} catch (err) {
    console.error("Failed to render graph:", err);
}
```

## Setup & Build
To build the WASM bundle, use:
```bash
wasm-pack build --target web
```

## CDN Usage

You can use CodeViz WASM directly in the browser via a CDN:

```javascript
<script type="module">
    import init, { parse } from 'https://unpkg.com/codeviz/codeviz_wasm.js';

    // Initialize the WASM module
    await init();

    // Parse source code and generate a diagram
    const diagram = parse("import os", "python", "module");
    console.log(diagram);
</script>
```
