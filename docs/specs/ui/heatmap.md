# Spec: Heatmap UI Layer (Phase 16)

## Overview
Integrates the `churn_score` and `primary_authors` metadata from the `CodeGraph` into
the React Flow Web UI.

## Requirements
- Add a "Heatmap Mode" toggle to the Web UI toolbar.
- When toggled ON, the `DependencyGraph` colors shift:
  - Low churn nodes = cool blue
  - High churn nodes = bright red (Hotspots)
- The detail side panel displays the list of `primary_authors` and the raw `churn_score`.
