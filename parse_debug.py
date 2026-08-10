# Ah!
# 15 events in a row, THEN diagram updated. This is correct debounce.
# But AFTER diagram updated at 38:39, MORE events keep coming in!
# Event triggered by main.rs
# Event triggered by main.rs
# diagram updated...
# Why are events for main.rs STILL arriving?
# 1. We did 5 writes spaced by 10ms. This should finish in 50ms.
# 2. Then we sleep 1500ms.
# During that 1500ms, run_core runs.
# BUT notify watcher might receive delayed events from the OS!
# Or wait, `run_core` uses `.codeviz_cache`.
# The cache might touch files? No, it writes meta.json in `.codeviz_cache`.
# What about the cache for `main.rs`? It writes `main.rs.json`? NO, it writes the hash of the file into `.codeviz_cache`?
# Let's check `CacheManager::put`.
