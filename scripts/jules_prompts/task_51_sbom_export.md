TASK: T51 — SBOM Export (CycloneDX / SPDX)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Export the CodeGraph as a Software Bill of Materials in CycloneDX 1.5
and SPDX 2.3 JSON formats.

Files to Create/Modify:
- `codeviz-core/src/render/sbom_cyclonedx.rs` [NEW]
- `codeviz-core/src/render/sbom_spdx.rs` [NEW]
- `codeviz-cli/src/main.rs` (extend `export` subcommand with `sbom-cyclonedx` / `sbom-spdx`)

Spec (READ ONLY):
  docs/specs/features/sbom_export.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CycloneDX output must be valid against the official JSON schema.
- No new external crate dependencies unless absolutely necessary.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
