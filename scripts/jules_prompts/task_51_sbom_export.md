TASK: T51 — SBOM Export (CycloneDX / SPDX)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement the ability to export the `CodeGraph` as a Software Bill of Materials
(SBOM) in industry-standard JSON formats (CycloneDX 1.5 and SPDX 2.3).
This is a critical compliance feature for enterprise adoption.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/sbom_export.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- The single-repo `CodeGraph` exists in `codeviz-core/src/graph.rs`.
- The CLI entry point is `codeviz-cli/src/main.rs`.
- The `export` subcommand already exists (from Task 27) but currently supports only JSON/Mermaid/Dot formats.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. CREATE: codeviz-core/src/render/sbom_cyclonedx.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement a function to convert a `CodeGraph` into a valid CycloneDX 1.5 JSON string.
Mapping rules:
- `Node.id` -> `component.bom-ref`
- `Node.label` -> `component.name`
- `Node.file_path` -> `component.version`
- `Edge (Imports)` -> `dependency.dependsOn`

Ensure the output is a valid JSON matching the official CycloneDX schema.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. CREATE: codeviz-core/src/render/sbom_spdx.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implement a function to convert a `CodeGraph` into a valid SPDX 2.3 JSON string.
Map the nodes to Packages/Files and edges to Relationships (`DEPENDS_ON`).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Extend the existing `export` subcommand to support the new format flags:
```bash
codeviz export --format sbom-cyclonedx --output sbom.json
codeviz export --format sbom-spdx --output sbom.spdx.json
```

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT use heavy 3rd-party crates to generate the SBOMs if possible; standard
   `serde_json` struct serialization is strongly preferred to keep the binary small.
2. The output MUST be valid against the respective schemas.
3. This is an `[ENT]` tier feature.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
5. Add unit tests verifying the JSON structures.

Commit: "jules: T51 — SBOM Export (CycloneDX and SPDX)"
Target branch: feat-t51-sbom-export
