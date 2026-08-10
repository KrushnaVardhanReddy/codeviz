# TASK: T62 — Graph Rendering Stress Test

Please implement the graph rendering stress test.

1. **Read Spec**: `docs/specs/features/qa_blind_spots.md`
2. **Context**: We need to benchmark React Flow performance for large repositories (>10,000 files).
3. **Execution**:
   - Write a script in `scripts/generate_stress_fixture.py` that generates a synthetic `CodeGraph` JSON fixture with 10,000 nodes and 20,000 edges.
   - Add a Playwright test in `codeviz-web/e2e/stress.spec.ts`.
   - The test should load this synthetic fixture and use Playwright to assert that the Time-To-Interactive (TTI) on the graph canvas remains under 5 seconds.
