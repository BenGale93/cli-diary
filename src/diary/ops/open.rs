//! # Open operations
//!
//! The open module contains functionality relating to the open command,
//! independent of the CLI.

use std::{io, path::PathBuf};

use chrono::prelude::*;

use crate::{errors::DiaryError, Diary};

/// The options available to the open command.
pub struct OpenFileOptions {
    /// The date of the entry to open.
    pub entry_date: DateTime<Local>,
}

/// Opens a specific diary entry for editing.
///
/// # Arguments
///
/// * `opts` - The options passed by the user at runtime.
/// * `diary` - Struct representing the diary.
/// * `user_input` - A function of type UserInput. A function that takes a file
/// and adds content to it.
pub fn open(
    opts: &OpenFileOptions,
    diary: &Diary,
    user_input: UserInput,
) -> Result<(), DiaryError> {
    let entry_path = diary.get_entry_path(&opts.entry_date);

    if !entry_path.exists() {
        return Err(DiaryError::NoEntry { source: None });
    }

    match user_input(entry_path) {
        Err(e) => Err(DiaryError::IOError(e)), // uncovered.
        _ => Ok(()),
    }
}

type UserInput = fn(P: PathBuf) -> io::Result<()>;

#[cfg(test)]
mod test {
    use std::{
        fs::{self, OpenOptions},
        io::{self, Write},
        path::PathBuf,
    };

    use chrono::{Local, TimeZone};

    use super::{open, OpenFileOptions};
    use crate::{
        config::Config,
        ops::{
            new::{new, NewOptions},
            testing,
        },
        utils::editing::test::test_string_getter,
        Diary,
    };

    fn test_user_input(filepath: PathBuf) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(filepath)?;

        let buf = "Test content";

        file.write_all(buf.as_bytes())
    }

    #[test]
    fn open_success() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());
        let diary = Diary::from_config(&config).unwrap();

        let new_opts = NewOptions { open: false };
        let entry_date = Local.with_ymd_and_hms(2021, 11, 6, 0, 0, 0).unwrap();

        new(&new_opts, &diary, &entry_date, test_string_getter).unwrap();

        let opts = OpenFileOptions { entry_date };
        open(&opts, &diary, test_user_input).unwrap();

        let entry_path = diary.get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
    }

    #[test]
    #[should_panic(expected = "value: NoEntry")]
    fn open_no_entry() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());
        let diary = Diary::from_config(&config).unwrap();

        let entry_date = Local.with_ymd_and_hms(2021, 11, 6, 0, 0, 0).unwrap();
        let opts = OpenFileOptions { entry_date };

        open(&opts, &diary, test_user_input).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn open_bad_config() {
        let config = Config::default();
        let diary = Diary::from_config(&config).unwrap();

        let entry_date = Local.with_ymd_and_hms(2021, 11, 6, 0, 0, 0).unwrap();
        let opts = OpenFileOptions { entry_date };

        open(&opts, &diary, test_user_input).unwrap();
    }
}
