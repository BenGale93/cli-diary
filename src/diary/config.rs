use std::{fs::File, io, path::PathBuf};

use chrono::prelude::*;

use crate::{
    errors::DiaryError,
    utils::file_system::{get_entry, get_entry_path},
};

pub struct ConfigBuilder {
    diary_path: PathBuf,
    prefix: String,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            diary_path: PathBuf::from(""),
            prefix: String::from("diary"),
        }
    }

    pub fn diary_path(mut self, diary_path: PathBuf) -> Self {
        self.diary_path = diary_path;
        self
    }
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn build(self) -> Config {
        let Self { diary_path, prefix } = self;
        Config { diary_path, prefix }
    }
}

/// A representation of the cli-diary config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    diary_path: PathBuf,
    prefix: String,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
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
    pub fn get_entry_path(&self, date: &Date<Local>) -> PathBuf {
        get_entry_path(self.diary_path.to_path_buf(), date, &self.prefix)
    }

    pub fn get_entry_file(&self, date: &Date<Local>) -> io::Result<File> {
        get_entry(self.diary_path.to_path_buf(), date, &self.prefix)
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
