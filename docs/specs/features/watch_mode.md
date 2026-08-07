# Spec: Watch Mode (`codeviz watch`)

## Purpose
Automatically re-runs `codeviz run` on file save during local development.
Provides a live feedback loop while coding.

---

## CLI
```
codeviz watch --path <dir> --output <file.md> [--diagram module|call|class]
```

---

## Behavior
1. Start watching `--path` recursively for file system events (create, modify, delete).
2. On any event matching a supported extension: debounce 300ms, then re-run.
3. On success: print `[14:32:01] ✅ Diagram updated — 12 nodes, 8 edges`.
4. On error: print `[14:32:01] ❌ Parse error in src/foo.py:14 — <message>`. Do NOT exit.
5. Continue watching after errors.
6. Ctrl+C exits cleanly with exit code 0.

---

## OS File Watching
Use the `notify` crate (cross-platform: inotify on Linux, FSEvents on macOS, ReadDirectoryChangesW on Windows).

---

## Debounce
Wait 300ms after the last file event before re-running.
If multiple files change within the debounce window, re-run once for all of them.

---

## Acceptance Criteria
- Saving a `.py` file triggers a diagram update within 400ms.
- Saving a `.txt` file does NOT trigger an update.
- A parse error prints the error but the watcher continues running.
- Ctrl+C exits with code 0.
- The process does not leave zombie threads after exit.
