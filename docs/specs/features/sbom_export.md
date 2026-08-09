# Spec: SBOM Export — CycloneDX / SPDX (Phase 21)

## Overview
Export the dependency graph as a Software Bill of Materials (SBOM) in industry-
standard formats. Required for US federal procurement (NIST EO 14028) and
increasingly mandated by enterprise security teams.

## Supported Formats
1. **CycloneDX 1.5** (JSON) — most widely adopted
2. **SPDX 2.3** (JSON) — required for some government contracts

## CLI Usage
```bash
codeviz export --format sbom-cyclonedx --output sbom.json
codeviz export --format sbom-spdx --output sbom.spdx.json
```

## Mapping: CodeGraph → SBOM
| CodeGraph field | CycloneDX field |
|---|---|
| `Node.id` | `component.bom-ref` |
| `Node.label` | `component.name` |
| `Node.file_path` | `component.version` (use git hash) |
| `Edge (Imports)` | `dependency.dependsOn` |

## Files to Modify
- `codeviz-core/src/render/` — add `sbom_cyclonedx.rs` and `sbom_spdx.rs`
- `codeviz-cli/src/main.rs` — extend `export` subcommand with new format flags

## Constraints
- This is `[ENT]` tier.
- SBOM output must be valid against the official CycloneDX JSON schema.
- `cargo test --all` and `cargo clippy --all` must pass.
