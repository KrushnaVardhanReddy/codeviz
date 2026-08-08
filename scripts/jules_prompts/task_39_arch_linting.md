TASK: T39 — Architectural Linting

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Enforce architectural boundaries by checking `CodeGraph` edges against rules defined in `codeviz.toml`.

Files to Modify/Create:
- `codeviz-core/src/config.rs` (add [rules] section)
- `codeviz-core/src/lint.rs` (evaluator)
- `codeviz-cli/src/main.rs` (enhance check mode)

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/arch_linting.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- Support glob matching for node paths.
- Return exit code 1 on violation.

═══════════════════════════════════════════════════════════════
IMPLEMENTATION TIPS
═══════════════════════════════════════════════════════════════
- Use the `glob` crate to match node IDs against the rules in the config.
- A rule `forbid = ["src/ui/** -> src/db/**"]` means if Node A matches `src/ui/**` and Node B matches `src/db/**`, and there is an edge A->B, it is a violation.
