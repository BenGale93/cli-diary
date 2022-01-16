//! # Open operations
//!
//! The open module contains functionality relating to the open command,
//! independent of the CLI.

use std::{io, path::PathBuf};

use chrono::prelude::*;

use crate::{config::Config, entry::Entry, errors::DiaryError};

/// The options available to the open command.
pub struct OpenFileOptions {
    /// The date of the entry to open.
    pub entry_date: Date<Local>,
}

/// Opens a specific diary entry for editing.
///
/// # Arguments
///
/// * `opts` - The options passed by the user at runtime.
/// * `config` - The contents of the config file.
/// * `user_input` - A function of type UserInput. A function that takes a file
/// and adds content to it.
pub fn open(
    opts: &OpenFileOptions,
    config: &Config,
    user_input: UserInput,
) -> Result<(), DiaryError> {
    config.initialised()?;

    let diary_entry = Entry::from_config(config)?;

    let entry_path = diary_entry.get_entry_path(&opts.entry_date);

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
        entry::Entry,
        ops::{
            new::{new, NewOptions},
            testing,
        },
        utils::editing::test::test_string_getter,
    };

    fn test_user_input(filepath: PathBuf) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(filepath)?;

        let buf = "Test content";

        file.write_all(buf.as_bytes())
    }

    #[test]
    fn open_success() {
        let config = testing::temp_config();
        testing::default_init(&config);

        let new_opts = NewOptions { open: false };
        let entry_date = Local.ymd(2021, 11, 6);

        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        let opts = OpenFileOptions { entry_date };
        open(&opts, &config, test_user_input).unwrap();

        let diary_file = Entry::from_config(&config).unwrap();

        let entry_path = diary_file.get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
    }

    #[test]
    #[should_panic(expected = "value: NoEntry")]
    fn open_no_entry() {
        let config = testing::temp_config();
        testing::default_init(&config);

        let entry_date = Local.ymd(2021, 11, 6);
        let opts = OpenFileOptions { entry_date };

        open(&opts, &config, test_user_input).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn open_bad_config() {
        let config = Config::default();

        let entry_date = Local.ymd(2021, 11, 6);
        let opts = OpenFileOptions { entry_date };

        open(&opts, &config, test_user_input).unwrap();
    }
}
