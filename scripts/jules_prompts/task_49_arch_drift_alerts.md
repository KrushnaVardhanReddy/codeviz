TASK: T49 — Architecture Drift Alerts (PR Comments & Slack)

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Extend the `codeviz check` CLI command to support cross-branch architectural
diffing (`--diff --base <branch>`), and integrate this into the GitHub Action
to automatically post PR comments and Slack notifications when architectural
drift (regressions) are detected.

Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/arch_drift_alerts.md

═══════════════════════════════════════════════════════════════
BACKGROUND — WHAT ALREADY EXISTS
═══════════════════════════════════════════════════════════════

- `codeviz-cli/src/main.rs` contains the `check` subcommand logic.
- `.github/workflows/codeviz.yml` (from Task 23) handles basic CI tasks.
- A Slack integration proxy exists or needs scaffolding in `codeviz-web/app/api/webhooks/slack/route.ts`.

═══════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
1. MODIFY: codeviz-cli/src/main.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Extend the `check` subcommand with two new optional flags:
- `--diff`: Enables differential checking.
- `--base <commit_ish>`: Specifies the base ref to compare against (defaults to `main`).

When `--diff` is passed, the CLI must:
1. Parse the current directory to generate the `CodeGraph` (head).
2. Git checkout the base ref (in memory or via a temporary worktree).
3. Parse the base directory to generate the base `CodeGraph`.
4. Compare the two graphs for regressions (new circular dependencies, health score drops > 20%, new violations).
5. Output the diff in a structured format (JSON or Markdown).

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
2. MODIFY: .github/workflows/codeviz.yml
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Update the GitHub Action to run `codeviz check --diff --base origin/main` on PRs.
If `codeviz check` exits with a failure code (indicating regressions) AND the
`GITHUB_TOKEN` is present:
- Use the `gh` CLI or a JS action script to post the CLI's Markdown output as a PR comment.
- If `CODEVIZ_SLACK_WEBHOOK_URL` is set in the environment, `curl` a payload to it.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
3. MODIFY: codeviz-web/app/api/webhooks/slack/route.ts
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
(If applicable/necessary per spec) Create or update the route to accept the incoming
webhook and forward it to Slack's API if you are building the proxy. If the CLI posts
directly to Slack, this may be minimal. Just follow the spec.

═══════════════════════════════════════════════════════════════
CRITICAL IMPLEMENTATION RULES
═══════════════════════════════════════════════════════════════
1. Do NOT add heavy new crates. Extend the existing CLI.
2. The GitHub Action must skip the comment step gracefully if `GITHUB_TOKEN` is missing.
3. The Slack notification is opt-in via environment variables.
4. Ensure `cargo test --all` and `cargo clippy --all -- -D warnings` pass.

Commit: "jules: T49 — Architecture Drift Alerts"
Target branch: feat-t49-arch-drift-alerts
