use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_e2e_dogfooding() {
    let mut cmd = Command::cargo_bin("codeviz-cli").unwrap();

    cmd.arg("run")
        .arg("--path")
        .arg(".")
        .arg("--output")
        .arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("codeviz-cli/tests/e2e_test.rs").or(predicate::str::contains("e2e_test.rs")));
}
