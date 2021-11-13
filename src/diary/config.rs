use std::path::PathBuf;

use crate::errors::DiaryError;

/// A representation of the cli-diary config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    diary_path: PathBuf,
    prefix: String,
}

impl Config {
    /// Creates a new Config struct based on the passed arguments.
    ///
    /// # Arguments
    ///
    /// * `diary_path` - The location of the diary folder.
    /// * `prefix` - The string prefix added before the date in diary entry filenames.
    pub fn new(diary_path: PathBuf, prefix: String) -> Self {
        Config { diary_path, prefix }
    }

    pub fn diary_path(&self) -> &PathBuf {
        &self.diary_path
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    pub fn initialised(&self) -> Result<(), DiaryError> {
        if self.diary_path == PathBuf::from("") {
            Err(DiaryError::UnInitialised { source: None })
        } else {
            Ok(())
        }
    }
}

impl ::std::default::Default for Config {
    /// Creates a default Config, used when the user doesn't have a config initialised.
    fn default() -> Self {
        Self {
            diary_path: PathBuf::from(""),
            prefix: String::from("diary"),
        }
    }
}
