pub mod init;
pub use self::init::{init, InitOptions};
pub mod add;
pub mod commit;
pub mod new;
pub mod open;

#[cfg(test)]
pub mod testing {
    use std::path::{Path, PathBuf};

    use chrono::prelude::*;
    use tempfile::tempdir;

    use super::{init, InitOptions};
    use crate::{
        config::Config,
        ops::new::{new, NewOptions},
        utils::editing::test::test_string_getter,
        Diary,
    };

    pub fn temp_path() -> PathBuf {
        tempdir().unwrap().path().to_path_buf()
    }

    pub fn temp_diary_path() -> PathBuf {
        let dir = temp_path();
        dir.join("diary")
    }

    pub fn temp_config() -> Config {
        let diary_dir = temp_diary_path();
        Config::builder().diary_path(diary_dir).build()
    }

    pub fn new_entry(config: &Config, entry_date: &DateTime<Local>) {
        let new_opts = NewOptions { open: false };
        let diary = Diary::from_config(config).unwrap();
        new(&new_opts, &diary, entry_date, test_string_getter).unwrap();
    }

    pub fn default_init(potential_path: &Path) {
        let init_opts = InitOptions {
            path: temp_path(),
            prefix: None,
            git_repo: false,
        };
        init(&init_opts, potential_path).unwrap();
    }
}
