# T65: Execution Flow (Entry Point) Visualization

You are responsible for implementing the React UI components required to support the new Execution Flow Visualization mode in the CodeViz Web UI.

## Context
We are adding a new view mode to `GraphCanvas.tsx` that shifts from a structural (Object-Oriented) visualization to an **Execution Flow** visualization. It starts from a user-selected "Entry Point" and shows a pure top-down call tree. 

## Core Requirements
Please read `docs/specs/09_execution_flow_view.md` carefully before beginning.

1. **GraphCanvas Toggle**: Add a state toggle for `primaryMode` (`'structural' | 'execution'`).
2. **Entry Point Selection**: Add a searchable dropdown in the UI (e.g., above the canvas or in the top navigation/sidebar) to select a root function.
3. **Smart Defaults**: Identify potential entry points automatically (e.g., functions with an in-degree of `0` on `Calls` edges, or functions matching `main`). Sort these at the top of the dropdown.
4. **In-Place Expansion**: In `execution` mode, display the tree using `dagre` with `rankdir: 'TB'` (Top-to-Bottom). Clicking a node should expand its immediate callees into the graph, progressively building a flowchart downward.

---

## Implementation Tips & Guidelines

### 1. Finding Entry Points (In-Degree Calculation)
To populate the entry point dropdown, you will need to scan the graph to find root functions:
- Iterate through all `Calls` edges and track `to_id` occurrences to compute the in-degree.
- Functions with an in-degree of `0` (they call other functions, but nothing calls them) are your primary candidates. 
- You can reuse the existing `resolveId()` helper in `GraphCanvas.tsx` to map edge targets to node IDs correctly.

### 2. State Management for the Tree
To implement the "in-place expansion" without blowing up the graph size:
- Add a new state: `const [expandedTreeNodes, setExpandedTreeNodes] = useState<Set<string>>(new Set());`
- When a user clicks a node in Execution Mode, toggle its ID in this Set.
- In your `useEffect` that builds the visible nodes, do a **Breadth-First Search (BFS)** starting from `selectedEntryPoint`. Only traverse down `Calls` edges if the current node's ID is in `expandedTreeNodes`.

### 3. Dagre Layout Adjustments
The Execution Flow should look like a classic flowchart, not a dense architecture map.
- When `viewMode === 'execution'`, call `dagre.layout` with `rankdir: 'TB'`.
- Increase `ranksep` (e.g., `120` or `140`) to give the vertical edges room to breathe.
- Turn off `Inherits` and `Instantiates` edges entirely in this mode to reduce visual noise. We only care about `Calls`.

### 4. Code Quality
- DO NOT remove or break the existing `'structural'` view modes (`'classes'`, `'expanded'`, `'focus'`). They must continue to work exactly as they do now.
- Keep the new dropdown UI clean and styled with Tailwind classes (using `bg-slate-800` / `bg-slate-900` to match the existing dark theme).
- Add informative comments above your BFS traversal logic.
