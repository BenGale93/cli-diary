//! # Add operations
//!
//! The add module contains functionality relating to the add command,
//! independent of the CLI.
use std::{fs::File, io::Write};

use chrono::prelude::*;

use crate::{errors::DiaryError, utils::editing, Diary, EntryContent};

/// The options available to the add command.
pub struct AddOptions<'a> {
    /// An optional entry tag.
    pub tag: Option<&'a str>,
}

/// Adds the given content to a file.
///
/// # Arguments
///
/// * `file` The file to add the content to.
/// * `content` The content to add to the file above.
/// * `tag` The optional tag to place above the content.
///
/// # Errors
///
/// * If the content provided is empty.
fn add_content(mut file: File, content: String, tag: Option<String>) -> Result<(), DiaryError> {
    if content.is_empty() {
        return Err(DiaryError::NoContent);
    }

    if let Some(tag) = tag {
        file.write_all(tag.as_bytes())?;
    }
    editing::add_user_content_to_file(&mut file, content)?;
    Ok(())
}

/// Adds user provided content to a diary entry.
///
/// # Arguments
///
/// * `opts` - The options available to the add function.
/// * `diary` - Struct representing the diary.
/// * `date` - The date of the entry to add to.
/// * `string_getter` - The function that obtains the string to add to the file.
///
/// # Returns
///
/// The unit if adding the content was successful, a DiaryError otherwise.
pub fn add(
    opts: &AddOptions,
    diary: &Diary,
    date: &Date<Local>,
    string_getter: editing::StringGetter,
) -> Result<(), DiaryError> {
    let file = match diary.get_entry_file(date) {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(DiaryError::NoEntry { source: Some(e) })
        }
        Err(e) => return Err(e.into()), // uncovered.
    };

    let content = string_getter("".to_owned())?;

    let tag_result = opts.tag.map(|tag| diary.file_type().tag(tag.to_owned()));

    add_content(file, content, tag_result)
}

#[cfg(test)]
mod test {
    use std::fs;

    use chrono::{Local, TimeZone};

    use crate::{
        ops::{
            add::{add, AddOptions},
            testing,
        },
        utils::editing::test::{test_empty_string_getter, test_string_getter},
        Diary,
    };
    #[test]
    fn add_no_tag() {
        let config = testing::temp_config();

        testing::default_init(config.diary_path());

        let entry_date = Local.ymd(2021, 11, 6);
        testing::new_entry(&config, &entry_date);

        let diary = Diary::from_config(&config).unwrap();

        let opts = AddOptions { tag: None };
        add(&opts, &diary, &entry_date, test_string_getter).unwrap();

        let diary_file = Diary::from_config(&config).unwrap();

        let entry_path = diary_file.get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
    }

    #[test]
    fn add_with_tag() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());

        let entry_date = Local.ymd(2021, 11, 6);
        testing::new_entry(&config, &entry_date);

        let diary = Diary::from_config(&config).unwrap();
        let opts = AddOptions { tag: Some("Tag") };
        add(&opts, &diary, &entry_date, test_string_getter).unwrap();

        let entry_path = diary.get_entry_path(&entry_date);

        let content = fs::read_to_string(entry_path).unwrap();

        assert!(content.contains("Test content"));
        assert!(content.contains("Tag"));
    }

    #[test]
    #[should_panic(expected = "value: NoContent")]
    fn add_empty_string() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());

        let entry_date = Local.ymd(2021, 11, 6);
        testing::new_entry(&config, &entry_date);

        let diary = Diary::from_config(&config).unwrap();
        let opts = AddOptions { tag: Some("Tag") };
        add(&opts, &diary, &entry_date, test_empty_string_getter).unwrap();
    }

    #[test]
    #[should_panic(expected = "value: NoEntry")]
    fn add_to_nonexistent_file() {
        let config = testing::temp_config();
        testing::default_init(config.diary_path());

        let diary = Diary::from_config(&config).unwrap();

        let entry_date = Local.ymd(2021, 11, 6);
        let opts = AddOptions { tag: Some("Tag") };
        add(&opts, &diary, &entry_date, test_string_getter).unwrap();
    }
}
