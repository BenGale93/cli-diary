use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub diary_path: PathBuf,
}

impl Config {
    pub fn new(diary_path: PathBuf) -> Self {
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
