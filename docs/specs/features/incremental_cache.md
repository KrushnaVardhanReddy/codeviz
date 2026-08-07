# Spec: Incremental Caching

## Purpose
Cache per-file `CodeGraph` parse results so unchanged files are not re-parsed on every run.
Targets pre-commit hook use-case: < 100ms for incremental runs on large repos.

---

## Cache Location
Default: `.codeviz_cache/` in the source root.
Configurable via `codeviz.toml`: `[cache] dir = ".codeviz_cache"`.

---

## Cache Key
Each file is cached individually. The cache key is:
```
SHA256(file_path + "|" + mtime_unix_ns + "|" + file_size_bytes)
```
If any component changes, the entry is invalidated and the file is re-parsed.

---

## Cache Entry Format
One JSON file per source file, named `<sha256_of_cache_key>.json`:
```json
{
  "cache_key": "<sha256>",
  "file_path": "src/parser.rs",
  "codeviz_version": "0.1.0",
  "nodes": [...],
  "edges": [...]
}
```

---

## Global Invalidation
The entire cache is invalidated (all entries deleted) when:
- `codeviz.toml` is modified
- The `codeviz` binary version changes (compare `codeviz_version` field)

---

## CLI Flags
```
codeviz run --no-cache    # bypass cache for this run
codeviz cache clear       # delete all cache entries
codeviz cache stats       # print hit/miss count for last run
```

---

## Acceptance Criteria
- Second run of `codeviz run` on an unchanged repo completes faster than first run.
- Modifying a single file invalidates only that file's cache entry.
- `--no-cache` flag produces identical output to a cold run.
- Cache directory is excluded from `codeviz` parsing automatically.
