use std::{
    fs::{create_dir, File},
    path::PathBuf,
    process::Command,
};

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod utils;

#[test]
fn test_commit_no_options() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "init",
        &dir_str,
        "-r",
    ]);
    cmd.assert();

    let mut cmd = Command::cargo_bin("diary")?;
    cmd.args(["--config", config_path.to_str().unwrap(), "new"]);
    cmd.assert();

    let mut cmd = Command::cargo_bin("diary")?;
    cmd.args(["--config", config_path.to_str().unwrap(), "commit"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Committed entry."));

    Ok(())
}

#[test]
fn test_commit_no_file() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "init",
        &dir_str,
        "-r",
    ]);
    cmd.assert();

    let mut cmd = Command::cargo_bin("diary")?;
    cmd.args(["--config", config_path.to_str().unwrap(), "commit"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_commit_given_date() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let (dir_str, config_path) = utils::create_temp_dir_and_path()?;

    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "init",
        &dir_str,
        "-r",
    ]);
    cmd.assert();

    let mut entry_path: PathBuf = [&dir_str, "diary/2020-01"].iter().collect();
    create_dir(&entry_path).unwrap();

    entry_path.push(PathBuf::from("diary_2020-01-01.md"));

    File::create(entry_path).unwrap();

    let mut cmd = Command::cargo_bin("diary")?;
    cmd.args([
        "--config",
        config_path.to_str().unwrap(),
        "commit",
        "-d",
        "2020-01-01",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Committed entry."));

    Ok(())
}
