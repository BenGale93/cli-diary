use std::path::PathBuf;

use crate::{
    diary_file::{self, DiaryFile},
    errors::DiaryError,
};

pub struct ConfigBuilder {
    diary_path: PathBuf,
    prefix: String,
    file_type: String,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            diary_path: PathBuf::from(""),
            prefix: String::from("diary"),
            file_type: String::from("md"),
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
    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = file_type.into();
        self
    }

    pub fn build(self) -> Config {
        let Self {
            diary_path,
            prefix,
            file_type,
        } = self;
        Config {
            diary_path,
            prefix,
            file_type,
        }
    }
}

/// A representation of the cli-diary config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    diary_path: PathBuf,
    prefix: String,
    file_type: String,
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

    pub fn file_type(&self) -> Result<impl diary_file::DiaryFile, DiaryError> {
        match self.file_type.as_str() {
            "md" => Ok(diary_file::MarkdownDiary::new(
                self.prefix.to_string(),
                self.diary_path.to_path_buf(),
            )),
            _ => Err(DiaryError::BadFileType),
        }
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
        ConfigBuilder::new().build()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Config;

    #[test]
    fn full_config_build() {
        let cfg = Config::builder()
            .diary_path(PathBuf::from("/home/"))
            .prefix("dy")
            .build();

        assert_eq!(cfg.prefix(), "dy");
        assert_eq!(cfg.diary_path(), &PathBuf::from("/home/"))
    }
}
