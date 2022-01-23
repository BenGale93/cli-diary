//! # New operations
//!
//! The new module contains functionality relating to the new command,
//! independent of the CLI.
use std::fs::OpenOptions;

use chrono::prelude::*;

use crate::{
    errors::DiaryError,
    utils::{editing, file_system},
    Diary, EntryContent,
};

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
/// * `diary` - Struct representing the diary.
/// * `date` - The date for which to create the new entry.
/// * `string_getter` - The function that obtains the string to add to the file.
///
/// # Returns
///
/// The unit upon successful creation of the entry.
/// DiaryError if the entry already exists.
/// DiaryError on any other IO issues.
pub fn new(
    opts: &NewOptions,
    diary: &Diary,
    date: &Date<Local>,
    string_getter: editing::StringGetter,
) -> Result<(), DiaryError> {
    let mut new_entry_path = file_system::month_folder(diary.diary_path(), date);
    file_system::create_month_folder(&new_entry_path)?;

    let entry_name = diary.file_name(date);

    new_entry_path.push(entry_name);
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(new_entry_path);

    let mut file = match result {
        Ok(mut file) => {
            editing::add_user_content_to_file(&mut file, diary.file_type().title(date))?;
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
    use std::fs;

    use chrono::prelude::*;

    use super::{new, NewOptions};
    use crate::{config::Config, ops::testing, utils::editing::test::test_string_getter, Diary};

    #[test]
    fn new_success() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());

        let diary = Diary::from_config(&config).unwrap();

        let new_opts = NewOptions { open: false };
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &diary, &date, test_string_getter).unwrap();

        let test_path = diary.get_entry_path(&date);

        assert!(test_path.exists());
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn new_not_init() {
        let config = testing::temp_config();
        let diary = Diary::from_config(&config).unwrap();

        let date = Local.ymd(2021, 11, 6);
        let new_opts = NewOptions { open: false };

        new(&new_opts, &diary, &date, test_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "kind: AlreadyExists")]
    fn new_fail_second_time() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());
        let diary = Diary::from_config(&config).unwrap();

        let new_opts = NewOptions { open: false };
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &diary, &date, test_string_getter).unwrap();
        new(&new_opts, &diary, &date, test_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn new_not_init_default_config() {
        let config = Config::default();
        let diary = Diary::from_config(&config).unwrap();

        let new_opts = NewOptions { open: false };
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &diary, &date, test_string_getter).unwrap();
    }

    #[test]
    fn new_open_file_success() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());
        let diary = Diary::from_config(&config).unwrap();

        let new_opts = NewOptions { open: true };
        let date = Local.ymd(2021, 11, 6);

        new(&new_opts, &diary, &date, test_string_getter).unwrap();

        let diary_file = Diary::from_config(&config).unwrap();

        let test_path = diary_file.get_entry_path(&date);

        let content = fs::read_to_string(test_path).unwrap();
        assert!(content.contains("Test content"));
    }
}
