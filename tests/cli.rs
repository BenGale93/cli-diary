use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod utils;

#[test]
fn test_help_text() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Add a new sub-entry"));

    Ok(())
}

#[test]
fn test_version_text() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn test_bad_command() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    cmd.arg("fake");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand 'fake'"));

    Ok(())
}
