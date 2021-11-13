//! # Add operations
//!
//! The add module contains functionality relating to the add command,
//! independent of the CLI.
use std::{
    fs::{File, OpenOptions},
    io,
    io::Write,
    path::PathBuf,
};

use chrono::{DateTime, Local};
use edit;

use crate::{
    errors::DiaryError,
    utils::{editing, file_system},
    Config,
};

/// The options available to the add command.
pub struct AddOptions<'a> {
    /// An optional entry tag.
    pub tag: Option<&'a str>,
}

/// Gets a diary entry file.
///
/// # Arguments
///
/// * `path` - The path to the diary entry.
/// * `date` - The date of the entry.
/// * `prefix` - The filename prefix.
fn get_entry(path: PathBuf, date: &DateTime<Local>, prefix: &str) -> io::Result<File> {
    let mut entry_path = file_system::month_folder(path, date);
    let entry_name = file_system::create_entry_name(prefix, date);
    entry_path.push(entry_name);
    return OpenOptions::new().append(true).open(entry_path);
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
/// * If
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
            Err(DiaryError::NoEntry { source: e })
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
pub fn add(opts: &AddOptions, config: &Config, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let file_result = get_entry(config.diary_path().to_path_buf(), date, config.prefix());

    let content = edit::edit("")?;

    add_content(file_result, content, opts.tag)
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use tempfile;

    use super::add_content;

    #[test]
    fn add_content_no_tag() {
        let tag = None;
        let content = "Test content".to_string();

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let temp_file = file.reopen();

        add_content(temp_file, content, tag).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, "Test content\n");
    }

    #[test]
    fn add_content_with_tag() {
        let tag = Some("Test");
        let content = "Test content".to_string();

        let mut file = tempfile::NamedTempFile::new().unwrap();

        let temp_file = file.reopen();

        add_content(temp_file, content, tag).unwrap();

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(buffer, "## Test\n\nTest content\n");
    }
    #[test]
    #[should_panic(expected = "value: NoContent")]
    fn add_content_empty_string() {
        let tag = Some("Test");
        let content = "".to_string();

        let file = tempfile::tempfile();

        add_content(file, content, tag).unwrap();
    }
}
