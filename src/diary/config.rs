#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    diary_path: String,
}

impl Config {
    pub fn new(diary_path: String) -> Self {
        Config { diary_path }
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            diary_path: String::from(""),
        }
    }
}
