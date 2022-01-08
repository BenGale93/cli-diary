use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod utils;

#[test]
fn test_new_success() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args(["--config", config_path.to_str().unwrap(), "init", &dir_str]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("diary")?;

    cmd.args(["--config", config_path.to_str().unwrap(), "new"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created today's entry."));

    let diary_path: PathBuf = [&dir_str, "diary"].iter().collect();

    assert!(diary_path.read_dir()?.count() == 1);

    Ok(())
}

#[test]
fn test_new_without_init() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (_, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args(["--config", config_path.to_str().unwrap(), "new"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Diary has not been initialised."));

    Ok(())
}
