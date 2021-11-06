use std::fs::OpenOptions;

use crate::{
    errors::DiaryError,
    utils::{editing, file_system},
    Config,
};
use chrono::{DateTime, Local};

pub struct NewOptions {
    pub open: bool,
}

pub fn new(opts: &NewOptions, config: &Config, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let mut new_entry_path = file_system::month_folder(config.diary_path().to_path_buf(), date);
    file_system::create_month_folder(&new_entry_path)?;

    let entry_name = file_system::create_entry_name(config.prefix(), date);

    new_entry_path.push(entry_name);
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(new_entry_path);

    let mut file = match result {
        Ok(mut file) => {
            editing::add_title(&mut file, date)?;
            file
        }
        Err(e) => return Err(e.into()),
    };
    if opts.open {
        editing::user_edit_file(&mut file)?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{ops::init, Config};
    use chrono::prelude::*;
    use init::InitOptions;
    use tempfile::tempdir;

    use super::{new, NewOptions};

    #[test]
    fn new_success() {
        let new_opts = NewOptions { open: false };
        let init_opts = InitOptions {
            path: PathBuf::from(""),
        };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::new(diary_path, String::from("diary"));
        let date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);

        init(&init_opts, &config).unwrap();

        new(&new_opts, &config, &date).unwrap();

        let mut test_path = config.diary_path().clone();
        test_path.push("2021-11");

        assert!(test_path.exists());

        test_path.push("diary_2021-11-06.md");

        assert!(test_path.exists());
    }

    #[test]
    #[should_panic]
    fn new_not_init() {
        let new_opts = NewOptions { open: false };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::new(diary_path, String::from("diary"));
        let date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);

        new(&new_opts, &config, &date).unwrap();
    }

    #[test]
    #[should_panic]
    fn new_fail_second_time() {
        let new_opts = NewOptions { open: false };
        let init_opts = InitOptions {
            path: PathBuf::from(""),
        };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::new(diary_path, String::from("diary"));
        let date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);

        init(&init_opts, &config).unwrap();

        new(&new_opts, &config, &date).unwrap();
        new(&new_opts, &config, &date).unwrap();
    }
}
