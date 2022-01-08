use std::{fs, path::PathBuf, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod utils;

#[test]
fn test_default_init_success() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args(["--config", config_path.to_str().unwrap(), "init", &dir_str]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialised diary."));

    let diary_path: PathBuf = [&dir_str, "diary"].iter().collect();

    assert!(diary_path.exists());
    assert!(config_path.exists());

    Ok(())
}

#[test]
fn test_init_success_with_options() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "init",
        &dir_str,
        "--prefix",
        "d",
        "--filetype",
        "rst",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialised diary."));

    let diary_path: PathBuf = [&dir_str, "diary"].iter().collect();

    assert!(diary_path.exists());

    let content = fs::read_to_string(config_path).expect("Unable to read file.");

    assert!(content.contains("prefix = 'd'"));
    assert!(content.contains("file_type = 'rst'"));

    Ok(())
}

#[test]
fn test_init_success_with_git_repo() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "init",
        &dir_str,
        "--repo",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialised diary."));

    let git_path: PathBuf = [&dir_str, "diary", ".git"].iter().collect();

    assert!(git_path.exists());

    Ok(())
}

#[test]
fn test_init_failure() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args(["--config", config_path.to_str().unwrap(), "init", &dir_str]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialised diary."));

    cmd.assert().failure();
    Ok(())
}
