//! # Open operations
//!
//! The open module contains functionality relating to the open command,
//! independent of the CLI.

use std::{io, path::PathBuf};

use chrono::{DateTime, Local};

use crate::{errors::DiaryError, utils::file_system, Config};

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
/// * `config` - The contents of the config file.
/// * `user_input` - A function of type UserInput. A function that takes a file
/// and adds content to it.
pub fn open(
    opts: &OpenFileOptions,
    config: &Config,
    user_input: UserInput,
) -> Result<(), DiaryError> {
    config.initialised()?;

    let entry_path = file_system::get_entry_path(
        config.diary_path().to_path_buf(),
        &opts.entry_date,
        config.prefix(),
    );

    if !entry_path.exists() {
        return Err(DiaryError::NoEntry { source: None });
    }

    match user_input(entry_path) {
        Err(e) => Err(DiaryError::IOError(e)),
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
    use tempfile::tempdir;

    use crate::{
        ops::new::{new, NewOptions},
        Config,
    };

    use super::{open, OpenFileOptions};

    fn test_user_input(filepath: PathBuf) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).open(filepath)?;

        let buf = "Test content";

        file.write_all(buf.as_bytes())
    }

    #[test]
    fn open_success() {
        let entry_date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);
        let opts = OpenFileOptions { entry_date };
        let temp_dir = tempdir().unwrap();
        let mut filepath = temp_dir.path().to_path_buf();

        let config = Config::new(filepath.clone(), "diary".to_string());

        let new_opts = NewOptions { open: false };

        new(&new_opts, &config, &entry_date).unwrap();

        open(&opts, &config, test_user_input).unwrap();

        filepath.push("2021-11/diary_2021-11-06.md");
        let content = fs::read_to_string(filepath).unwrap();

        assert!(content.contains("Test content"));
    }

    #[test]
    #[should_panic(expected = "value: NoEntry")]
    fn open_no_entry() {
        let entry_date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);
        let opts = OpenFileOptions { entry_date };
        let temp_dir = tempdir().unwrap();
        let filepath = temp_dir.path().to_path_buf();

        let config = Config::new(filepath, "diary".to_string());

        open(&opts, &config, test_user_input).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: UnInitialised")]
    fn open_bad_config() {
        let entry_date = Local.ymd(2021, 11, 6).and_hms(0, 0, 0);
        let opts = OpenFileOptions { entry_date };

        let config = Config::new(PathBuf::from(""), "diary".to_string());

        open(&opts, &config, test_user_input).unwrap();
    }
}
