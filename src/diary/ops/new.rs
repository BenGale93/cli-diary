//! # New operations
//!
//! The new module contains functionality relating to the new command,
//! independent of the CLI.
use crate::{
    diary_file::DiaryFile,
    errors::DiaryError,
    utils::{editing, file_system},
    Config,
};
use chrono::prelude::*;
use std::fs::OpenOptions;

/// The options available to the new command.
pub struct NewOptions {
    /// Whether or not to open the new entry for an initial entry.
    pub open: bool,
}

/// Creates a new diary entry.
///
/// # Arguments
///
/// * `opts` - The options passed by the user at runtime.
/// * `config` - The contents of the config file.
/// * `date` - The date for which to create the new entry.
///
/// # Returns
///
/// The unit upon successful creation of the entry.
/// DiaryError if the entry already exists.
/// DiaryError on any other IO issues.
pub fn new(
    opts: &NewOptions,
    config: &Config,
    date: &Date<Local>,
    string_getter: editing::StringGetter,
) -> Result<(), DiaryError> {
    config.initialised()?;

    let diary_file = DiaryFile::from_config(config)?;

    let mut new_entry_path = file_system::month_folder(config.diary_path().to_path_buf(), date);
    file_system::create_month_folder(&new_entry_path)?;

    let entry_name = diary_file.file_name(date);

    new_entry_path.push(entry_name);
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(new_entry_path);

    let mut file = match result {
        Ok(mut file) => {
            editing::add_user_content_to_file(&mut file, diary_file.file_type().title(date))?;
            file
        }
        Err(e) => return Err(e.into()),
    };
    if opts.open {
        let contents = string_getter("".to_string())?;
        editing::add_user_content_to_file(&mut file, contents)?;
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, path::PathBuf};

    use crate::{diary_file::DiaryFile, ops::init, utils::editing::test_string_getter, Config};
    use chrono::prelude::*;
    use init::InitOptions;
    use tempfile::tempdir;

    use super::{new, NewOptions};

    #[test]
    fn new_success() {
        let new_opts = NewOptions { open: false };
        let init_opts = InitOptions {
            path: PathBuf::from(""),
            prefix: None,
            git_repo: false,
        };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::builder().diary_path(diary_path).build();
        let date = Local.ymd(2021, 11, 6);

        init(&init_opts, &config).unwrap();

        new(&new_opts, &config, &date, test_string_getter).unwrap();

        let diary_file = DiaryFile::from_config(&config).unwrap();

        let test_path = diary_file.get_entry_path(&date);

        assert!(test_path.exists());
    }

    #[test]
    #[should_panic(expected = "kind: NotFound")]
    fn new_not_init() {
        let new_opts = NewOptions { open: false };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::builder().diary_path(diary_path).build();
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &config, &date, test_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "kind: AlreadyExists")]
    fn new_fail_second_time() {
        let new_opts = NewOptions { open: false };
        let init_opts = InitOptions {
            path: PathBuf::from(""),
            prefix: None,
            git_repo: false,
        };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::builder().diary_path(diary_path).build();
        let date = Local.ymd(2021, 11, 6);

        init(&init_opts, &config).unwrap();

        new(&new_opts, &config, &date, test_string_getter).unwrap();
        new(&new_opts, &config, &date, test_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn new_not_init_default_config() {
        let new_opts = NewOptions { open: false };
        let config = Config::default();
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &config, &date, test_string_getter).unwrap();
    }

    #[test]
    fn new_open_file_success() {
        let new_opts = NewOptions { open: true };
        let init_opts = InitOptions {
            path: PathBuf::from(""),
            prefix: None,
            git_repo: false,
        };
        let diary_path = tempdir().unwrap().path().to_path_buf();
        let config = Config::builder().diary_path(diary_path).build();
        let date = Local.ymd(2021, 11, 6);

        init(&init_opts, &config).unwrap();

        new(&new_opts, &config, &date, test_string_getter).unwrap();

        let diary_file = DiaryFile::from_config(&config).unwrap();

        let test_path = diary_file.get_entry_path(&date);

        let content = fs::read_to_string(test_path).unwrap();
        assert!(content.contains("Test content"));
    }
}
