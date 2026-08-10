use predicates::prelude::*;
use std::fs;
use std::io::Write;
use std::time::Duration;
use tempfile::tempdir;

#[cfg(unix)]
#[test]
fn test_watch_mode_debounce_and_error_handling() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("main.rs");

    // Initial file
    fs::write(&file_path, "fn main() {}").unwrap();

    // We need to run it in the background and interact with it.
    // assert_cmd::Command is basically a wrapper around std::process::Command.
    let child = std::process::Command::new(assert_cmd::cargo::cargo_bin("codeviz-cli"))
        .arg("watch")
        .arg("--path")
        .arg(dir.path())
        .arg("--output")
        .arg(dir.path().join("out.md"))
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn watch process");

    std::thread::sleep(Duration::from_millis(500)); // wait for it to start

    // 1. Debounce logic: multiple writes
    for _ in 0..5 {
        let mut file = std::fs::OpenOptions::new().append(true).open(&file_path).unwrap();
        writeln!(file, "// some change").unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }

    std::thread::sleep(Duration::from_millis(1500)); // Wait for debounce and run

    // 2. Parse Error logic
    let bad_file_path = dir.path().join("bad.rs");
    fs::write(&bad_file_path, "fn invalid() { {").unwrap();

    std::thread::sleep(Duration::from_millis(1500)); // Wait for debounce and run

    // Shutdown signal
    unsafe {
        libc::kill(child.id() as i32, libc::SIGINT);
    }

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("STDOUT:\n{}", stdout);

    // Verify debounce: there should not be 5 updates for the rapid file changes.
    // The total updates should be 1 (initial) + 1 (debounced 5 saves) + 1 (bad file) = 3 or so.
    let update_count = stdout.matches("✅ Diagram updated").count();
    assert!(update_count <= 4, "Expected few updates due to debounce, got {}", update_count);

    // Verify format
    assert!(predicate::str::is_match(r"\[\d{2}:\d{2}:\d{2}\] ✅ Diagram updated — \d+ nodes, \d+ edges").unwrap().eval(&stdout));

    // Verify parse error format
    assert!(predicate::str::is_match(r"\[\d{2}:\d{2}:\d{2}\] ❌ Parse error in .*bad\.rs.* —").unwrap().eval(&stdout));
}
