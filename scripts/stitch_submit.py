#!/usr/bin/env python3
"""
Stitch Remote MCP Submitter for CodeViz
Interacts with the Google Stitch MCP server via mcp-remote proxy to generate UI screens.

Usage:
  python3 stitch_submit.py --list
  python3 stitch_submit.py --create "Project Name"
  python3 stitch_submit.py --project-info <project_id>
  python3 stitch_submit.py --list-tasks
  python3 stitch_submit.py --submit-task <task_num>
  python3 stitch_submit.py --submit-batch 11
"""

import subprocess
import json
import time
import sys
import os

# ──────────────────────────────────────────────────────────────────────────────
# Config & API Key Resolution
# ──────────────────────────────────────────────────────────────────────────────

def _load_api_key():
    key = os.environ.get("STITCH_API_KEY")
    if key: return key

    for envfile in [".env.local", ".env"]:
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), envfile)
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("STITCH_API_KEY="):
                        return line.split("=", 1)[1].strip()

    mcp_config_path = os.path.expanduser("~/.gemini/antigravity/mcp_config.json")
    if os.path.exists(mcp_config_path):
        try:
            with open(mcp_config_path) as f:
                content = f.read()
            import re
            cleaned_content = re.sub(r'(?<!http:)(?<!https:)//.*', '', content)
            config = json.loads(cleaned_content)
            stitch_config = config.get("mcpServers", {}).get("stitch", {})
            headers = stitch_config.get("headers", {})
            key = headers.get("X-Goog-Api-Key") or headers.get("x-goog-api-key")
            if key: return key
        except Exception:
            pass

    key = os.environ.get("JULES_API_KEY")
    if key: return key

    for envfile in [".env.local", ".env"]:
        path = os.path.join(os.path.dirname(os.path.abspath(__file__)), envfile)
        if os.path.exists(path):
            with open(path) as f:
                for line in f:
                    line = line.strip()
                    if line.startswith("JULES_API_KEY="):
                        return line.split("=", 1)[1].strip()

    print("❌ API key not found in environment, .env files, or mcp_config.json")
    sys.exit(1)


API_KEY = _load_api_key()

# ──────────────────────────────────────────────────────────────────────────────
# Stitch MCP Client Wrapper
# ──────────────────────────────────────────────────────────────────────────────

class StitchMCPClient:
    def __init__(self):
        print("🔗 Connecting to Stitch MCP Server via mcp-remote...")
        self.proc = subprocess.Popen(
            ["npx", "-y", "mcp-remote", "https://stitch.googleapis.com/mcp", "--header", f"X-Goog-Api-Key: {API_KEY}"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        time.sleep(6)
        if self.proc.poll() is not None:
            print("❌ Failed to start mcp-remote proxy!")
            sys.exit(1)
        self.request_id = 1
        self._initialize_handshake()

    def _initialize_handshake(self):
        init_req = {
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "stitch-client", "version": "1.0.0"}
            },
            "id": self.request_id
        }
        self._send_raw(init_req)
        if self._read_raw():
            self._send_raw({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def _send_raw(self, msg):
        self.proc.stdin.write(json.dumps(msg) + "\n")
        self.proc.stdin.flush()

    def _read_raw(self, timeout=10.0):
        import select
        r, _, _ = select.select([self.proc.stdout], [], [], timeout)
        if r:
            return json.loads(self.proc.stdout.readline())
        return None

    def call_tool(self, name, arguments=None):
        self.request_id += 1
        self._send_raw({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {"name": name, "arguments": arguments or {}},
            "id": self.request_id
        })
        while True:
            line = self.proc.stdout.readline()
            if not line: return None
            try:
                data = json.loads(line)
                if data.get("id") == self.request_id: return data
            except Exception:
                pass

    def close(self):
        self.proc.terminate()
        self.proc.wait()


# ──────────────────────────────────────────────────────────────────────────────
# CodeViz UI Tasks (Stitch)
# ──────────────────────────────────────────────────────────────────────────────

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def _load_prompt(relative_path):
    full_path = os.path.join(REPO_ROOT, relative_path)
    if not os.path.exists(full_path):
        print(f"❌ Prompt file not found: {full_path}")
        sys.exit(1)
    with open(full_path) as f:
        return f.read()

CODEVIZ_UI_TASKS = {
    28: {
        "name": "Task 28 — Web UI Scaffold (Next.js + Tailwind)",
        "batch": 11,
        "prompt_file": "scripts/stitch_prompts/task_28_web_ui_scaffold.txt",
    },
    29: {
        "name": "Task 29 — Architecture Overview Graph (React Flow)",
        "batch": 11,
        "prompt_file": "scripts/stitch_prompts/task_29_architecture_graph.txt",
    },
    32: {
        "name": "Task 32 — CFG Side Panel in Web UI",
        "batch": 12,
        "prompt_file": "scripts/stitch_prompts/task_32_cfg_side_panel.txt",
    },
}

def submit_stitch_task(task_num):
    if task_num not in CODEVIZ_UI_TASKS:
        print(f"❌ Stitch task {task_num} not found. Available: {list(CODEVIZ_UI_TASKS.keys())}")
        sys.exit(1)

    task = CODEVIZ_UI_TASKS[task_num]
    prompt_content = _load_prompt(task["prompt_file"])

    print(f"\n🎨 Stitch Task [{task_num}]: {task['name']} (Batch {task['batch']})")
    client = StitchMCPClient()
    try:
        project_title = f"CodeViz {task['name']}"
        res = client.call_tool("create_project", {"title": project_title})
        if not res or "result" not in res:
            sys.exit(1)
        
        project_id = json.loads(res["result"]["content"][0]["text"])["name"].split("/")[-1]
        print(f"  ✅ Project created: ID = {project_id}")

        print(f"  🚀 Generating screen from prompt...")
        res2 = client.call_tool("generate_screen_from_text", {
            "projectId": project_id,
            "modelId": "GEMINI_3_1_PRO",
            "deviceType": "DESKTOP",
            "prompt": prompt_content,
        })
        
        data2 = json.loads(res2["result"]["content"][0]["text"])
        print(f"\n  ✅ Stitch task submitted successfully!")
        for component in data2.get("outputComponents", []):
            for screen in component.get("design", {}).get("screens", []):
                if url := screen.get("htmlCode", {}).get("downloadUrl"):
                    print(f"  • Screen: {screen.get('title', 'Untitled')} | URL: {url}")
    finally:
        client.close()


def main():
    args = sys.argv[1:]
    if not args or "--help" in args:
        print(__doc__)
        sys.exit(0)

    if "--list-tasks" in args:
        print("\n📋 CodeViz UI Tasks (Stitch):\n")
        for num, task in CODEVIZ_UI_TASKS.items():
            print(f"  [{num}] {task['name']} (Batch {task['batch']})")
        sys.exit(0)

    if "--submit-task" in args:
        idx = args.index("--submit-task")
        submit_stitch_task(int(args[idx + 1]))
        sys.exit(0)

    if "--submit-batch" in args:
        idx = args.index("--submit-batch")
        batch = int(args[idx + 1])
        for num, task in CODEVIZ_UI_TASKS.items():
            if task["batch"] == batch:
                submit_stitch_task(num)
        sys.exit(0)

if __name__ == "__main__":
    main()
