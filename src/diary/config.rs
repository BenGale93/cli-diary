use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    diary_path: PathBuf,
    prefix: String,
}

impl Config {
    pub fn new(diary_path: PathBuf, prefix: String) -> Self {
        Config { diary_path, prefix }
    }

    pub fn diary_path(&self) -> &PathBuf {
        &self.diary_path
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            diary_path: PathBuf::from(""),
            prefix: String::from("diary"),
        }
    }
}
