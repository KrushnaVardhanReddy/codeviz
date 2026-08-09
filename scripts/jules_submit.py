#!/usr/bin/env python3
"""
Jules Batch Submitter for CodeViz
===================================
Submits coding tasks to Jules (https://jules.google.com).
Prompts live in scripts/jules_prompts/. Specs live in docs/specs/.

Usage:
  python scripts/jules_submit.py --list
  python scripts/jules_submit.py --task 1
  python scripts/jules_submit.py --batch 1
  python scripts/jules_submit.py --batch 2 --branch feature/python-parser
"""

import json
import urllib.request
import urllib.error
import sys
import os

# ─────────────────────────────────────────────────────────────
# Auth
# ─────────────────────────────────────────────────────────────

def _load_api_key():
    key = os.environ.get("JULES_API_KEY")
    if key:
        return key
    for envfile in [".env.local", ".env"]:
        repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        path = os.path.join(repo_root, envfile)
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("JULES_API_KEY="):
                        return line.split("=", 1)[1].strip()
    print("❌ JULES_API_KEY not found. Add to .env.local: JULES_API_KEY=<key>")
    sys.exit(1)

API_KEY  = _load_api_key()
API_URL  = "https://jules.googleapis.com/v1alpha/sessions"

# ─────────────────────────────────────────────────────────────
# Project config
# ─────────────────────────────────────────────────────────────

REPO_ROOT   = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO_SOURCE = "sources/github/KrushnaVardhanReddy/codeviz"

BRANCH = "main"
if "--branch" in sys.argv:
    idx = sys.argv.index("--branch")
    if idx + 1 < len(sys.argv):
        BRANCH = sys.argv[idx + 1]

# ─────────────────────────────────────────────────────────────
# Safety rules prepended to every prompt
# ─────────────────────────────────────────────────────────────

SAFETY_RULES = """
MANDATORY RULES — VIOLATION = REJECTED PR:
1. NEVER stub, mock, or TODO core logic. Every function must be fully implemented.
2. All Rust code must compile cleanly with `cargo build` (no warnings allowed).
3. Every public function must have a doc-comment (`///`).
4. No `unwrap()` in library code — use `?` or explicit error handling.
5. Commit message must start with "jules: " prefix.
6. Include comprehensive unit tests (`#[cfg(test)] mod tests { ... }`) for all new logic.
7. COMMUNICATION PROTOCOL: Whenever you ask a question or pause to report progress, you MUST prefix your message with the Task Number and Name (e.g. "TASK: T## — [Name]").

Project: CodeViz
Tech stack: Rust (workspace), Tree-sitter, wasm-pack, JSON-RPC (MCP)
Spec-first: ALWAYS read the referenced docs/specs/ file before writing any code.
Architecture: codeviz-core / codeviz-cli / codeviz-wasm / codeviz-mcp crates
""".strip()

# ─────────────────────────────────────────────────────────────
# Task registry — name + phase + slug only. All detail in .md files.
# ─────────────────────────────────────────────────────────────

TASKS = {
    # Phase 0: Foundation
     1: {"name": "01 — Rust Workspace Init",        "phase": "Phase 0: Foundation",          "slug": "task_01_rust_workspace_init"},
     2: {"name": "02 — CodeGraph IR Structs",        "phase": "Phase 0: Foundation",          "slug": "task_02_codegraph_ir"},
     3: {"name": "03 — LanguageParser Trait",        "phase": "Phase 0: Foundation",          "slug": "task_03_language_parser_trait"},
     4: {"name": "04 — Mermaid Renderer",            "phase": "Phase 0: Foundation",          "slug": "task_04_mermaid_renderer"},
     5: {"name": "05 — Safe Markdown Injection",     "phase": "Phase 0: Foundation",          "slug": "task_05_markdown_injection"},
    # Phase 1: Python
     6: {"name": "06 — Python Parser",               "phase": "Phase 1: Python",              "slug": "task_06_python_parser"},
     7: {"name": "07 — CLI: codeviz run",            "phase": "Phase 1: Python",              "slug": "task_07_cli_run"},
     8: {"name": "08 — GitHub Actions CI",           "phase": "Phase 1: Python",              "slug": "task_08_github_actions_ci"},
    # Phase 2: TypeScript + WASM
     9: {"name": "09 — TypeScript/JS Parser",        "phase": "Phase 2: TypeScript + WASM",   "slug": "task_09_typescript_parser"},
    10: {"name": "10 — WASM Adapter",                "phase": "Phase 2: TypeScript + WASM",   "slug": "task_10_wasm_adapter"},
    # Phase 3: MCP Server
    11: {"name": "11 — MCP Server Core",             "phase": "Phase 3: MCP Server",          "slug": "task_11_mcp_server"},
    12: {"name": "12 — MCP Integration Tests",       "phase": "Phase 3: MCP Server",          "slug": "task_12_mcp_integration_tests"},
    # Phase 4: Go
    13: {"name": "13 — Go Parser",                   "phase": "Phase 4: Go",                  "slug": "task_13_go_parser"},
    # Phase 5: Rust Language Parser
    14: {"name": "14 — Rust Parser",                 "phase": "Phase 5: Rust",                "slug": "task_14_rust_parser"},
    # Phase 6: Java / Kotlin
    15: {"name": "15 — Java Parser",                 "phase": "Phase 6: Java/Kotlin",         "slug": "task_15_java_parser"},
    25: {"name": "25 — Kotlin Parser",               "phase": "Phase 6: Java/Kotlin",         "slug": "task_25_kotlin_parser"},
    # Phase 7: V1.0 Core Features
    16: {"name": "16 — codeviz.toml Config",         "phase": "Phase 7: V1.0",                "slug": "task_16_config"},
    26: {"name": "26 — Multiple Output Targets",     "phase": "Phase 7: V1.0",                "slug": "task_26_multiple_outputs"},
    # Phase 8: Critical Features
    18: {"name": "18 — Check Mode",                  "phase": "Phase 8: Critical Features",   "slug": "task_18_check_mode"},
    19: {"name": "19 — Incremental Cache",           "phase": "Phase 8: Critical Features",   "slug": "task_19_incremental_cache"},
    21: {"name": "21 — Install Hook",                "phase": "Phase 8: Critical Features",   "slug": "task_21_install_hook"},
    # Phase 9: Developer UX
    22: {"name": "22 — Watch Mode",                  "phase": "Phase 9: Developer UX",        "slug": "task_22_watch_mode"},
    20: {"name": "20 — Graph Diff",                  "phase": "Phase 9: Developer UX",        "slug": "task_20_graph_diff"},
    27: {"name": "27 — JSON/DOT Export",             "phase": "Phase 9: Developer UX",        "slug": "task_27_export_formats"},
    # Phase 10: Distribution
    23: {"name": "23 — GitHub Actions Marketplace",  "phase": "Phase 10: Distribution",       "slug": "task_23_github_action"},
    24: {"name": "24 — npm WASM Package",            "phase": "Phase 10: Distribution",       "slug": "task_24_npm_package"},
    # Phase 11: Web UI
    28: {"name": "28 — Web UI Setup (Next.js + React Flow)", "phase": "Phase 11: Web UI",    "slug": "task_28_webui_setup"},
    29: {"name": "29 — DependencyGraph Viewer",      "phase": "Phase 11: Web UI",             "slug": "task_29_dependency_graph_viewer"},
    # Phase 12: Control Flow Graph
    30: {"name": "30 — CFG IR Extension",            "phase": "Phase 12: CFG",               "slug": "task_30_cfg_ir"},
    31: {"name": "31 — CFG Parser Emitters",         "phase": "Phase 12: CFG",               "slug": "task_31_cfg_parsers"},
    32: {"name": "32 — CFG Web UI Renderer",         "phase": "Phase 12: CFG",               "slug": "task_32_cfg_webui"},
    # Phase 13: Auth
    33: {"name": "33A — Auth Core: GitHub & Google OAuth", "phase": "Phase 13: Auth",              "slug": "task_33a_auth_core"},
    330: {"name": "33B — Auth DB Adapter & E2E", "phase": "Phase 13: Auth",                        "slug": "task_33b_auth_e2e"},
    # Phase 14: Teams
    34: {"name": "34 — Team Workspaces & Repo Groups","phase": "Phase 14: Teams",            "slug": "task_34_teams"},
    # Phase 15: Enterprise
    35: {"name": "35 — Enterprise SSO & Audit Logs", "phase": "Phase 15: Enterprise",        "slug": "task_35_enterprise_sso"},
    # Phase 16: Enterprise Insights
    36: {"name": "36 — Git History Integration",     "phase": "Phase 16: Insights",          "slug": "task_36_git_history"},
    37: {"name": "37 — Blast Radius Analysis",       "phase": "Phase 16: Insights",          "slug": "task_37_blast_radius"},
    38: {"name": "38 — Heatmap UI Layer",            "phase": "Phase 16: Insights",          "slug": "task_38_heatmap_ui"},
    39: {"name": "39 — Architectural Linting",       "phase": "Phase 16: Insights",          "slug": "task_39_arch_linting"},
    # Phase 17: Universal Parser
    40: {"name": "40 — Query-Based Universal Parser",  "phase": "Phase 17: Universal",         "slug": "task_40_universal_parser"},
    # Phase 18: Advanced Analysis
    41: {"name": "41 — Circular Dependency Detection", "phase": "Phase 18: Advanced",          "slug": "task_41_circular_deps"},
    42: {"name": "42 — Unused Module Detection",       "phase": "Phase 18: Advanced",          "slug": "task_42_unused_modules"},
    43: {"name": "43 — PageRank & Centrality",         "phase": "Phase 18: Advanced",          "slug": "task_43_pagerank"},
    44: {"name": "44 — Code Health Score",             "phase": "Phase 18: Advanced",          "slug": "task_44_code_health"},
    45: {"name": "45 — Code Coverage Overlay",         "phase": "Phase 18: Advanced",          "slug": "task_45_code_coverage"},
    # Phase 19: Semantic Search
    46: {"name": "46 — Semantic Code Search (LanceDB)",  "phase": "Phase 19: Semantic",          "slug": "task_46_semantic_search"},
    # MVP v1: Viral OSS Features
    47: {"name": "47 — VS Code Extension",               "phase": "MVP v1: Viral",               "slug": "task_47_vscode_extension"},
    48: {"name": "48 — summarize_architecture MCP Tool", "phase": "MVP v1: Viral",               "slug": "task_48_mcp_summarize"},
    53: {"name": "53 — Interactive Call Path Explorer",  "phase": "MVP v1: Viral",               "slug": "task_53_call_path_explorer"},
    55: {"name": "55 — MCP Debugging Tools",             "phase": "MVP v1: Viral",               "slug": "task_55_mcp_debug_tools"},
    # MVP v2: Sticky Team Features
    49: {"name": "49 — Architecture Drift Alerts",       "phase": "MVP v2: Sticky",              "slug": "task_49_arch_drift_alerts"},
    52: {"name": "52 — Onboard Me Walkthrough",          "phase": "MVP v2: Sticky",              "slug": "task_52_onboard_me"},
    # MVP v3: Enterprise Revenue Features
    54: {"name": "54 — OpenTelemetry Trace Overlay",     "phase": "MVP v3: Enterprise",          "slug": "task_54_otel_trace_overlay"},
    50: {"name": "50 — Multi-Repo Cross-Service Graph",  "phase": "MVP v3: Enterprise",          "slug": "task_50_multi_repo"},
    51: {"name": "51 — SBOM Export (CycloneDX/SPDX)",   "phase": "MVP v3: Enterprise",          "slug": "task_51_sbom_export"},
}

# ─────────────────────────────────────────────────────────────
# Batches
# ─────────────────────────────────────────────────────────────

BATCHES = {
    1:  {"desc": "Phase 0: Foundation",                    "tasks": [1, 2, 3, 4, 5]},
    2:  {"desc": "Phase 1: Python Parser + CLI + CI",      "tasks": [6, 7, 8]},
    3:  {"desc": "Phase 2: TypeScript + WASM",             "tasks": [9, 10]},
    4:  {"desc": "Phase 3: MCP Server + Tests",            "tasks": [11, 12]},
    5:  {"desc": "Phase 4 & 5: Go + Rust Parsers",         "tasks": [13, 14]},
    6:  {"desc": "Phase 6: Java + Kotlin Parsers",         "tasks": [15, 25]},
    7:  {"desc": "Phase 7: Config + Universal Parser",     "tasks": [16, 26, 40]},
    8:  {"desc": "Phase 8: Critical Features",             "tasks": [18, 19, 21]},
    9:  {"desc": "Phase 9: Developer UX",                  "tasks": [22, 20, 27]},
    10: {"desc": "Phase 10: Distribution (GitHub Action + npm WASM)",  "tasks": [23, 24]},
    11: {"desc": "Phase 11: Web UI (Next.js + React Flow)",            "tasks": [28, 29]},
    12: {"desc": "Phase 12: Control Flow Graph",                       "tasks": [30, 31, 32]},
    13: {"desc": "Phase 13: Auth (GitHub + Google OAuth)",             "tasks": [33]},
    14: {"desc": "Phase 14: Teams & Workspaces",                       "tasks": [34]},
    15: {"desc": "Phase 15: Enterprise SSO & Audit Logs",              "tasks": [35]},
    16: {"desc": "Phase 16: Enterprise Insights",                      "tasks": [36, 37, 38, 39]},
    18: {"desc": "Phase 18: Advanced Analysis (Cycles, PageRank, Health)", "tasks": [41, 42, 43, 44, 45]},
}

# ─────────────────────────────────────────────────────────────
# Prompt loading & building
# ─────────────────────────────────────────────────────────────

def _load_prompt(slug: str) -> str:
    path = os.path.join(REPO_ROOT, f"scripts/jules_prompts/{slug}.md")
    if not os.path.exists(path):
        print(f"  ⚠️  Prompt file not found: scripts/jules_prompts/{slug}.md")
        sys.exit(1)
    with open(path) as f:
        return f.read()

def _build_prompt(task: dict) -> str:
    return SAFETY_RULES + "\n\n---\n\n" + _load_prompt(task["slug"])

# ─────────────────────────────────────────────────────────────
# API
# ─────────────────────────────────────────────────────────────

def _post_session(full_prompt: str, label: str):
    payload = json.dumps({
        "prompt": full_prompt,
        "sourceContext": {
            "source": REPO_SOURCE,
            "githubRepoContext": {"startingBranch": BRANCH}
        }
    }).encode()
    req = urllib.request.Request(
        API_URL,
        data=payload,
        headers={"Content-Type": "application/json", "x-goog-api-key": API_KEY},
        method="POST"
    )
    try:
        with urllib.request.urlopen(req) as resp:
            result = json.loads(resp.read())
            session_id = result.get("name", "unknown").split("/")[-1]
            print(f"  ✅ {session_id}  →  https://jules.google.com/session/{session_id}")
            return session_id
    except urllib.error.HTTPError as e:
        print(f"  ❌ HTTP {e.code} [{label}]: {e.read().decode()}")
        return None

# ─────────────────────────────────────────────────────────────
# Commands
# ─────────────────────────────────────────────────────────────

def submit_task(task_num: int):
    task = TASKS[task_num]
    print(f"🚀 [{task_num:>2}] {task['name']}  (branch: {BRANCH})")
    _post_session(_build_prompt(task), task["name"])

def submit_batch(batch_num: int):
    batch = BATCHES[batch_num]
    print(f"\n📦 Batch {batch_num}: {batch['desc']}  |  branch: {BRANCH}\n")
    for t in batch["tasks"]:
        task = TASKS[t]
        print(f"  → [{t:>2}] {task['name']}")
        _post_session(_build_prompt(task), task["name"])

def list_tasks():
    print("\n📋 CodeViz Jules Tasks\n")
    # Sort tasks: group by phase (using BATCHES order), then by task num
    batch_order = [t for b in BATCHES.values() for t in b["tasks"]]
    sorted_nums = sorted(TASKS.keys(), key=lambda n: (batch_order.index(n) if n in batch_order else 999, n))
    current_phase = None
    for num in sorted_nums:
        task = TASKS[num]
        if task["phase"] != current_phase:
            print(f"\n  ── {task['phase']} ──")
            current_phase = task["phase"]
        print(f"    [{num:>2}] {task['name']}")
    print("\n\n📦 Batches:\n")
    for num, batch in BATCHES.items():
        print(f"  {num:>2}: {batch['desc']}")
        print(f"       tasks: {batch['tasks']}")
    print()


def main():
    args = sys.argv[1:]
    if not args or "--list" in args:
        list_tasks()
    elif "--batch" in args:
        submit_batch(int(args[args.index("--batch") + 1]))
    elif "--task" in args:
        submit_task(int(args[args.index("--task") + 1]))
    else:
        print("Usage:")
        print("  python scripts/jules_submit.py --list")
        print("  python scripts/jules_submit.py --task <N>")
        print("  python scripts/jules_submit.py --batch <N> [--branch <branch>]")

if __name__ == "__main__":
    main()
