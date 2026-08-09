# Spec: Architecture Drift Alerts — PR Comments & Slack (Phase 20)

## Overview
When `codeviz check` runs in CI and detects architectural drift (new circular
dependencies, health score drop >20%, new violations), automatically:
1. Post a comment on the GitHub Pull Request with a graph diff.
2. Optionally send a Slack webhook notification.

## GitHub PR Comment Format
```
## 🏗️ CodeViz Architecture Report

⚠️ **2 new issues detected:**
- 🔴 New circular dependency: `src/auth.rs` ↔ `src/db.rs`
- 🟡 Health score dropped: `src/parser.rs` 8.2 → 6.1

✅ **No regressions in 14 other modules.**

<details><summary>Full graph diff</summary>
{mermaid diff here}
</details>
```

## GitHub Action Integration
Extend the existing GitHub Action (Task 23) to:
1. Run `codeviz check --diff --base main`.
2. If issues found, post a PR comment using `GITHUB_TOKEN`.
3. Optionally call `CODEVIZ_SLACK_WEBHOOK_URL` env var with a Slack payload.

## Files to Modify/Create
- `codeviz-cli/src/main.rs` — add `--diff` and `--base` flags to `check` subcommand
- `.github/workflows/codeviz.yml` — add PR comment step
- `codeviz-web/app/api/webhooks/slack/route.ts` — Slack webhook proxy (optional)

## Constraints
- The GitHub comment step must be skipped gracefully if `GITHUB_TOKEN` is not set.
- Slack notification is opt-in via env var.
- No new Rust crates. Extend existing `check` subcommand.
- `cargo test --all` and `cargo clippy --all` must pass.
