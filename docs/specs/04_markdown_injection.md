# Spec: Markdown Injection

## Purpose
`inject_mermaid` safely updates the CodeViz diagram in a markdown file
without touching any content outside the sentinel tags.

---

## Sentinel Tags
```
<!-- CODEVIZ_START -->
<!-- CODEVIZ_END -->
```
Both tags must appear exactly once, on their own line.

---

## API
```rust
pub fn inject_mermaid(markdown: &str, mermaid: &str) -> Result<String, InjectError>;

pub enum InjectError {
    MissingTags,    // neither sentinel tag found
    MalformedTags,  // only one tag found, or START appears after END
}
```

---

## Injection Format
The content injected between the tags must be:
```
<!-- CODEVIZ_START -->
```mermaid
<mermaid content here>
```
<!-- CODEVIZ_END -->
```
Exactly one blank line is NOT required — the fence immediately follows the start tag line.

---

## Preservation Rules
- All content **before** `<!-- CODEVIZ_START -->` is preserved byte-for-byte.
- All content **after** `<!-- CODEVIZ_END -->` is preserved byte-for-byte.
- The sentinel tag lines themselves are preserved.

---

## Error Cases
| Condition | Error |
|---|---|
| Neither tag present | `InjectError::MissingTags` |
| Only `CODEVIZ_START` present | `InjectError::MalformedTags` |
| Only `CODEVIZ_END` present | `InjectError::MalformedTags` |
| `CODEVIZ_END` appears before `CODEVIZ_START` | `InjectError::MalformedTags` |

---

## Acceptance Criteria
- Calling `inject_mermaid` twice with the same mermaid string produces the same result as calling it once (idempotent).
- Content before and after sentinel tags is 100% preserved (verified byte-for-byte).
- An empty `mermaid` string injects an empty fenced block — does not panic.
- Tags may appear anywhere in the document (not just at the end).
