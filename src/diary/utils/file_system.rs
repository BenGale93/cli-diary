use std::{
    fs::{create_dir, File, OpenOptions},
    io,
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
                Err(DiaryError::UnInitialised { source: Some(e) })
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

pub fn get_entry_path(path: PathBuf, date: &DateTime<Local>, prefix: &str) -> PathBuf {
    let mut entry_path = month_folder(path, date);
    let entry_name = create_entry_name(prefix, date);
    entry_path.push(entry_name);
    entry_path
}

/// Gets a diary entry file.
///
/// # Arguments
///
/// * `path` - The path to the diary entry.
/// * `date` - The date of the entry.
/// * `prefix` - The filename prefix.
pub fn get_entry(path: PathBuf, date: &DateTime<Local>, prefix: &str) -> io::Result<File> {
    let entry_path = get_entry_path(path, date, prefix);
    return OpenOptions::new().append(true).open(entry_path);
}
