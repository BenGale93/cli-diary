use std::path::PathBuf;

use clap::crate_name;

use crate::errors::DiaryError;

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

    #[must_use]
    pub fn diary_path(mut self, diary_path: PathBuf) -> Self {
        self.diary_path = diary_path;
        self
    }
    #[must_use]
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }
    #[must_use]
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

    pub fn file_type(&self) -> &String {
        &self.file_type
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

#[derive(Default)]
pub struct ConfigManager {
    config: Config,
    location: Option<PathBuf>,
}

impl ConfigManager {
    pub fn location(&self) -> &Option<PathBuf> {
        &self.location
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn with_location(location: Option<PathBuf>) -> ConfigManager {
        ConfigManager {
            location,
            ..Default::default()
        }
    }

    pub fn read(mut self) -> Result<ConfigManager, confy::ConfyError> {
        let config: Config = match &self.location {
            Some(l) => confy::load_path(l)?,
            _ => confy::load(crate_name!())?,
        };
        self.config = config;

        Ok(self)
    }

    pub fn write(self) -> Result<(), confy::ConfyError> {
        match self.location {
            Some(l) => confy::store_path(l, self.config),
            _ => confy::store(crate_name!(), self.config),
        }
    }

    #[must_use]
    pub fn update_config(mut self, config: Config) -> ConfigManager {
        self.config = config;
        self
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
