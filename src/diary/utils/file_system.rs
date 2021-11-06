use std::{
    fs::create_dir,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};

use crate::errors::DiaryError;

pub fn month_folder(path_root: PathBuf, date: &DateTime<Local>) -> PathBuf {
    let month_folder = PathBuf::from(date.format("%Y-%m").to_string());
    [path_root, month_folder].iter().collect()
}

pub fn create_month_folder(path: &Path) -> Result<(), DiaryError> {
    if !path.exists() {
        match create_dir(&path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(DiaryError::UnInitialised { source: e })
            }
            Err(e) => panic!("Unexpected IO error {}", e),
        }
    } else {
        Ok(())
    }
}

pub fn create_entry_name(prefix: &str, date: &DateTime<Local>) -> PathBuf {
    let entry_suffix = date.format("%Y-%m-%d").to_string();
    PathBuf::from(format!("{}_{}.md", prefix, entry_suffix))
}
