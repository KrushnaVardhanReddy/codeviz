import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Does the notify event paths contain the cache file?
# `manager.put` saves to a file in `.codeviz_cache/<hash>.json`.
# `["rs", "py", ...].contains(ext)`
# But wait! Look at `run_core`:
# It writes `out.md`!
# Is it possible that `tempdir` writes everything to `/tmp/...`? Yes.
# Is it possible that some other process is touching files?
# Wait! In the test, we write `// some change` 5 times.
# Total lines added: 5.
# But `Event triggered by ["/tmp/.../main.rs"]` happens ~20 times!
# Why are there so many events for main.rs?
# Maybe `notify` on linux gives `Open`, `Access`, `Modify`, `CloseWrite`, etc.
# And `rx.recv_timeout(50)` processes them one by one.
# But they arrive very fast!
# If there are 20 events, `recv_timeout` reads one, updates `last_event_time`, loops.
# It reads 20 events in <20ms total!
# Then `rx` is empty. It waits 300ms.
# THEN WHY does it say:
# [23:38:39] ✅ Diagram updated
# Event triggered
# Event triggered
# [23:38:40] ✅ Diagram updated
# Event triggered
# [23:38:40] ✅ Diagram updated

# Ah! If `run_core` takes e.g. 50ms.
# Could `run_core` be generating an event? NO, we established it writes `.md` and `.json`.
# What about the test process?
# Look at `watch_test.rs`:
# for _ in 0..5 {
#     let mut file = std::fs::OpenOptions::new().append(true).open(&file_path).unwrap();
#     writeln!(file, "// some change").unwrap();
#     std::thread::sleep(Duration::from_millis(10));
# }
# Maybe the test process itself is slow? No, 5*10ms = 50ms.

# WAIT!
# I noticed `run_core` is calling `registry.parse_file`.
# Does `parse_file` modify the file? No.
# Could the cache clearing or something modify the file? No.
# What if `notify` events are buffered in the OS, and delivered in chunks?
# Yes, FSEvents/inotify can deliver events delayed.
# But 300ms is a LONG time. It shouldn't be delivered 1 SECOND later!

# Wait! Is it possible `std::thread::sleep(Duration::from_millis(1500))` allows the 5 loops to finish, then we wait 1.5s, then write `bad.rs`, wait 1.5s...
# Why are there 9 updates?
# Let's count the updates in the output:
# 39: update
# 40: update
# 40: update
# 40: update
# 41: update (nodes 4, so bad.rs arrived)
# 41: update
# 41: update
# 42: update
# 42: update

# 9 updates!
# Each update takes ~300ms because it waits for the debounce.
# It seems `notify` is sending an event, debounce expires, it runs. THEN `notify` sends ANOTHER event for the SAME file!
# Why would `notify` send another event after 300ms for a file that was modified 300ms ago??
# Ah! In `run_watch`:
# We do `let mut watcher = notify::recommended_watcher(tx)`.
# notify watcher sends `EventKind::Access(AccessKind::Close(AccessMode::Write))`!
# Wait! Does `run_core` READ `main.rs`?
# YES! `std::fs::read_to_string(&file)` !!!
# When `run_core` READS `main.rs`, `notify` on Linux might emit an `Access` or `Close(Read)` event!
# And what does our code do?
# `if ["rs", "py", ...].contains(ext)`!
# It triggers on ALL events, even READ events!
