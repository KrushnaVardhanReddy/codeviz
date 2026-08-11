# Interactive Graph Legend & Filtering

Provide a way for users to filter the CodeViz Architecture Graph by clicking on the Legend items. This will allow users to hide/show specific node kinds (e.g., Classes, Functions) and edge kinds (e.g., Calls, Imports), reducing visual clutter in complex graphs.

## Top-to-Bottom Auto Layout
In addition to filtering, the graph will automatically layout nodes in a top-to-bottom flow (TD) using the `dagre` layout engine, rather than relying on a naive grid layout.

## Proposed Changes

We will move the static `Legend` component into the `GraphCanvas` so it can control the graph state, and we will update `Legend.tsx` to handle click events.

### `codeviz-web/components/Legend.tsx`
- Accept new props: `hiddenNodeKinds: Set<string>`, `hiddenEdgeKinds: Set<string>`, `onToggleNodeKind: (kind: string) => void`, `onToggleEdgeKind: (kind: string) => void`.
- Add click handlers to each legend item (span).
- Apply visual dimming (`opacity-30`) to items that are currently hidden.

### `codeviz-web/components/GraphCanvas.tsx`
- Integrate `dagre` to automatically layout `nodesToRender` in a `TD` (Top-to-Down) direction based on `edgesToRender`.
- Add state hooks for `hiddenNodeKinds` and `hiddenEdgeKinds`.
- Create toggle functions to add/remove kinds from these sets.
- Render the `<Legend />` component directly inside `GraphCanvas`.
- Modify `nodesToRender` to filter out nodes whose `kind` is in `hiddenNodeKinds`.
- Modify `edgesToRender` to filter out edges whose `kind` is in `hiddenEdgeKinds`.
- Ensure any edges connected to a hidden node are also filtered out.

### `codeviz-web/app/page.tsx`
- Remove the standalone `<Legend />` component, as it is now integrated within `GraphCanvas`.

## Verification Plan

### Automated Tests
- Run `npm run build` to ensure type-checking passes.

### Manual Verification
- Open the application locally (Dashboard and Playground).
- Verify the legend is visible at the bottom of the graph.
- Verify that the layout is a tree-like top-to-bottom layout instead of a 3-column grid.
- Click "Calls" in the legend: verify the "Calls" edges disappear and the layout readjusts.
