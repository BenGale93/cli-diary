use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::{tempdir, NamedTempFile};

mod utils;

#[test]
fn test_default_init_success() -> utils::TestReturn {
    let mut cmd = Command::cargo_bin("diary")?;

    let dir = tempdir()?;
    let file = NamedTempFile::new()?;

    let dir_str = dir.path().to_str().unwrap();
    let file_dir = file.path().to_owned();
    file.close()?;

    cmd.args(["--config", file_dir.to_str().unwrap(), "init", dir_str]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialised diary."));

    let diary_path: PathBuf = [dir_str, "diary"].iter().collect();

    assert!(diary_path.exists());

    Ok(())
}
