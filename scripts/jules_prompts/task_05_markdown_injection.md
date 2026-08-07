# Jules Task 05 — Safe Markdown Injection

## Objective
Implement the safe markdown injector in `codeviz-core`.
It updates ONLY the content between `<!-- CODEVIZ_START -->` and `<!-- CODEVIZ_END -->` sentinel tags.

## Files to Create/Modify
- `codeviz-core/src/inject.rs`
- `codeviz-core/src/lib.rs` (re-export)

## Requirements
```rust
pub fn inject_mermaid(markdown: &str, mermaid: &str) -> Result<String, InjectError>
```
- If both sentinel tags exist: replace content between them with the new Mermaid block.
- If no sentinel tags exist: return `Err(InjectError::MissingTags)`.
- If only one tag exists: return `Err(InjectError::MalformedTags)`.
- The injected block must be wrapped with a newline, triple-backtick mermaid fence, the diagram content, closing triple-backtick fence, and a trailing newline.
- All content OUTSIDE the sentinel tags must be 100% preserved.

## Unit Tests
Write tests covering:
1. Normal injection — content is replaced, outside content preserved
2. Missing both tags — returns `MissingTags` error
3. Only one tag — returns `MalformedTags` error
4. Idempotency — calling `inject_mermaid` twice produces the same output as calling it once
5. Empty mermaid string — injects an empty fenced block without panicking
