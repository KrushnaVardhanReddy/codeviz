TASK: T19 — Incremental Caching

═══════════════════════════════════════════════════════════════
OBJECTIVE
═══════════════════════════════════════════════════════════════
Implement per-file caching per the spec:
- SHA256 cache key strategy
- JSON cache entry format with `codeviz_version`
- Global invalidation on config or binary version change
- `--no-cache`, `cache clear`, `cache stats` CLI support

Files to Modify/Create:


Spec (READ ONLY — implement from it, never edit):
  docs/specs/features/incremental_cache.md

═══════════════════════════════════════════════════════════════
CONSTRAINTS & RULES
═══════════════════════════════════════════════════════════════
- CONTEXT: Use a fast hashing algorithm (like `blake3` or `sha2`) to hash file contents. Store the cache in a local `.codeviz/` directory.
- Write comprehensive unit tests:
- Write a cache entry, read it back, assert equality
- Modify mtime → assert cache miss
- Assert `--no-cache` bypasses cache entirely
- No unwraps or panics in core parsing logic. Return Result.
- Ensure 'cargo clippy --all -- -D warnings' and 'cargo test --all' pass cleanly.
