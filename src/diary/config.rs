use std::{fs::canonicalize, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub diary_path: PathBuf,
}

impl Config {
    pub fn new(diary_path: PathBuf) -> Self {
        let diary_path = canonicalize(diary_path).unwrap();
        Config { diary_path }
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            diary_path: PathBuf::from(""),
        }
    }
}
