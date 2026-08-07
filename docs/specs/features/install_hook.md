# Spec: Install Hook (`codeviz install-hook`)

## Purpose
Zero-friction setup command that configures the pre-commit hook and markdown sentinel tags
automatically. Removes all manual YAML editing.

---

## CLI
```
codeviz install-hook [--path <dir>] [--output <file.md>]
```

---

## Steps Performed
1. **Detect pre-commit**: check if `pre-commit` is installed (`which pre-commit`).
   - If not found: print instructions to install it, continue (hook still written).
2. **Update `.pre-commit-config.yaml`**:
   - If file doesn't exist: create it.
   - If file exists but codeviz hook is not present: append the entry.
   - If codeviz hook already present: print `Already configured.` and skip.
3. **Update `--output` markdown file**:
   - If sentinel tags already present: print `Sentinel tags already present.` and skip.
   - If absent: append the sentinel tags block at the end of the file.
   - If file doesn't exist: create it with the sentinel tags.
4. **Print a summary** of what was changed.

---

## pre-commit Hook Entry
```yaml
repos:
  - repo: local
    hooks:
      - id: codeviz
        name: Update architecture diagram
        entry: codeviz run --path ./src --output README.md
        language: system
        pass_filenames: false
        always_run: false
```

---

## Sentinel Tags Block (appended to output file)
```markdown

---

## Architecture

<!-- CODEVIZ_START -->
<!-- CODEVIZ_END -->
```

---

## Acceptance Criteria
- Running on a fresh repo creates both `.pre-commit-config.yaml` and adds sentinel tags.
- Running twice is idempotent — no duplicate entries.
- Works even if `pre-commit` is not installed (only shows a warning).
- Respects `--output` flag to target the correct markdown file.
