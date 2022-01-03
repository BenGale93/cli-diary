use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod utils;

#[test]
fn test_help_text() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Add a new sub-entry"));

    Ok(())
}
