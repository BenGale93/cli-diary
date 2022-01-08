use std::{error, path::PathBuf};

use tempfile::{tempdir, NamedTempFile};

pub type TestReturn = Result<(), Box<dyn error::Error>>;

#[allow(dead_code)]
pub fn create_temp_dir_and_path() -> Result<(String, PathBuf), Box<dyn error::Error>> {
    let dir_str = tempdir()?.path().to_str().unwrap().to_string();

    let config_file = NamedTempFile::new()?;
    let config_path = config_file.path().to_owned();
    config_file.close()?;

    Ok((dir_str, config_path))
}
