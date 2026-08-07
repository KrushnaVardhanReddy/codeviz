# Jules Task 19 — Incremental Caching

## Spec
Read `docs/specs/features/incremental_cache.md` before writing any code.

## Files to Create/Modify
- `codeviz-core/src/cache.rs` (new)
- `codeviz-cli/src/main.rs` (integrate cache into `run` and add `cache` subcommand)

## Requirements
Implement per-file caching per the spec:
- SHA256 cache key strategy
- JSON cache entry format with `codeviz_version`
- Global invalidation on config or binary version change
- `--no-cache`, `cache clear`, `cache stats` CLI support

## Unit Tests
- Write a cache entry, read it back, assert equality
- Modify mtime → assert cache miss
- Assert `--no-cache` bypasses cache entirely
