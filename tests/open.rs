use assert_cmd::Command;
use predicates::prelude::*;

mod utils;

#[test]
fn test_open_failure_no_init() -> utils::TestReturn {
    let (_, config_path) = utils::create_temp_dir_and_path()?;

    let mut cmd = Command::cargo_bin("diary")?;

    cmd.args(["--config", config_path.to_str().unwrap(), "open"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Diary has not been initialised."));

    Ok(())
}

#[test]
fn test_open_failure_no_new() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args(["--config", config_path.to_str().unwrap(), "init", &dir_str]);
    cmd.assert().success();

    let mut cmd = Command::cargo_bin("diary")?;

    cmd.args(["--config", config_path.to_str().unwrap(), "open"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not been found."));

    Ok(())
}
