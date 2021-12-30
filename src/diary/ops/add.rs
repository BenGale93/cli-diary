//! # Add operations
//!
//! The add module contains functionality relating to the add command,
//! independent of the CLI.
use std::{fs::File, io, io::Write};

use chrono::prelude::*;

use crate::{diary_file::DiaryFile, errors::DiaryError, utils::editing, Config};

/// The options available to the add command.
pub struct AddOptions<'a> {
    /// An optional entry tag.
    pub tag: Option<&'a str>,
}

/// Adds the given content to a file.
///
/// # Arguments
///
/// * `file_result` The prospective file to add the content to.
/// * `content` The content to add to the file above.
/// * `tag` The optional tag to place above the content, header 2 markdown.
///
/// # Errors
///
/// * If `file_result` contains an error.
/// * If the content provided is empty.
fn add_content(
    file_result: io::Result<File>,
    content: String,
    tag: Option<&str>,
) -> Result<(), DiaryError> {
    if content.is_empty() {
        return Err(DiaryError::NoContent);
    }

    match file_result {
        Ok(mut file) => {
            if let Some(tag) = tag {
                let markdown_tag = format!("## {}\n\n", tag);
                file.write_all(markdown_tag.as_bytes())?;
            }
            editing::add_user_content_to_file(&mut file, content)?;
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(DiaryError::NoEntry { source: Some(e) })
        }
        Err(e) => Err(e.into()),
    }
}

/// Adds user provided content to a diary entry.
///
/// # Arguments
///
/// * `opts` - The options available to the add function.
/// * `config` - The diary config file.
/// * `date` - The date of the entry to add to.
///
/// # Returns
///
/// The unit if adding the content was successful, a DiaryError otherwise.
pub fn add(
    opts: &AddOptions,
    config: &Config,
    date: &Date<Local>,
    string_getter: editing::StringGetter,
) -> Result<(), DiaryError> {
    let file_result = config.file_type()?.get_entry(date);

    let content = string_getter("".to_string())?;

    add_content(file_result, content, opts.tag)
}

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};
    use std::fs;
    use tempfile::tempdir;

    use crate::{
        diary_file::DiaryFile,
        ops::{
            add::{add, AddOptions},
            new::{new, NewOptions},
        },
        utils::editing::{test_empty_string_getter, test_string_getter},
        Config,
    };
    #[test]
    fn add_no_tag() {
        let entry_date = Local.ymd(2021, 11, 6);
        let opts = AddOptions { tag: None };
        let temp_dir = tempdir().unwrap();
        let filepath = temp_dir.path().to_path_buf();

        let config = Config::builder().diary_path(filepath).build();

        let new_opts = NewOptions { open: false };

        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        add(&opts, &config, &entry_date, test_string_getter).unwrap();

        let entry_path = config.file_type().unwrap().get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
    }

    #[test]
    fn add_with_tag() {
        let entry_date = Local.ymd(2021, 11, 6);
        let opts = AddOptions { tag: Some("Tag") };
        let temp_dir = tempdir().unwrap();
        let filepath = temp_dir.path().to_path_buf();

        let config = Config::builder().diary_path(filepath).build();

        let new_opts = NewOptions { open: false };

        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        add(&opts, &config, &entry_date, test_string_getter).unwrap();

        let entry_path = config.file_type().unwrap().get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
        assert!(content.contains("Tag"));
    }
    #[test]
    #[should_panic(expected = "value: NoContent")]
    fn add_empty_string() {
        let entry_date = Local.ymd(2021, 11, 6);
        let opts = AddOptions { tag: Some("Tag") };
        let temp_dir = tempdir().unwrap();
        let filepath = temp_dir.path().to_path_buf();

        let config = Config::builder().diary_path(filepath).build();

        let new_opts = NewOptions { open: false };

        new(&new_opts, &config, &entry_date, test_string_getter).unwrap();

        add(&opts, &config, &entry_date, test_empty_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: NoEntry")]
    fn add_to_nonexistent_file() {
        let entry_date = Local.ymd(2021, 11, 6);
        let opts = AddOptions { tag: Some("Tag") };
        let temp_dir = tempdir().unwrap();
        let filepath = temp_dir.path().to_path_buf();

        let config = Config::builder().diary_path(filepath).build();

        add(&opts, &config, &entry_date, test_string_getter).unwrap();
    }
}
