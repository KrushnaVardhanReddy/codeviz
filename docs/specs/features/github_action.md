# Spec: GitHub Actions Marketplace Action

## Purpose
Make CodeViz adoptable with 3 lines of YAML — no binary installation required.

---

## Usage (for end users)
```yaml
# .github/workflows/docs.yml
- uses: KrushnaVardhanReddy/codeviz@v1
  with:
    path: ./src
    output: README.md
    diagram: module
```

---

## action.yml
```yaml
name: 'CodeViz'
description: 'Auto-generate architecture diagrams from source code'
inputs:
  path:
    description: 'Source directory to scan'
    default: '.'
  output:
    description: 'Markdown file to update'
    default: 'README.md'
  diagram:
    description: 'Diagram type: module | call | class'
    default: 'module'
  commit:
    description: 'Commit updated diagram automatically (true/false)'
    default: 'true'
runs:
  using: 'composite'
  steps:
    - name: Download codeviz binary
      shell: bash
      run: |
        LATEST=$(curl -s https://api.github.com/repos/KrushnaVardhanReddy/codeviz/releases/latest | jq -r '.tag_name')
        curl -L "https://github.com/KrushnaVardhanReddy/codeviz/releases/download/$LATEST/codeviz-linux" -o /usr/local/bin/codeviz
        chmod +x /usr/local/bin/codeviz
    - name: Run codeviz
      shell: bash
      run: codeviz run --path "${{ inputs.path }}" --output "${{ inputs.output }}" --diagram "${{ inputs.diagram }}"
    - name: Commit updated diagram
      if: inputs.commit == 'true'
      shell: bash
      run: |
        git config user.name "github-actions[bot]"
        git config user.email "github-actions[bot]@users.noreply.github.com"
        git add "${{ inputs.output }}"
        git diff --staged --quiet || git commit -m "docs: update architecture diagram [skip ci]"
        git push
```

---

## Acceptance Criteria
- The action runs successfully on a public test repository.
- Diagram is committed back to the repo if `commit: true`.
- `commit: false` skips the git step (useful when using `check` mode instead).
- Works on `ubuntu-latest` and `macos-latest`.
