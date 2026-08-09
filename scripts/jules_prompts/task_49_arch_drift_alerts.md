TASK: T49 — Architecture Drift Alerts (PR Comments + Slack)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
When `codeviz check` detects architectural drift in CI, automatically post
a summary comment on the GitHub PR and optionally notify a Slack webhook.

Files to Modify/Create:
- `codeviz-cli/src/main.rs`         (add `--diff` and `--base` flags to `check`)
- `.github/workflows/codeviz.yml`   (add PR comment step)
- `codeviz-web/app/api/webhooks/slack/route.ts` [NEW] (Slack proxy, optional)

Spec (READ ONLY):
  docs/specs/features/arch_drift_alerts.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- PR comment step must be silently skipped if `GITHUB_TOKEN` is not set.
- Slack notification is opt-in via `CODEVIZ_SLACK_WEBHOOK_URL` env var.
- No new Rust crates. Extend existing `check` subcommand only.
- Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.
