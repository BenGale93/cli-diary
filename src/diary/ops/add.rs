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

pub struct AddOptions<'a> {
    pub tag: Option<&'a str>,
}

fn get_entry(path: PathBuf, date: &DateTime<Local>, prefix: &str) -> io::Result<File> {
    let mut entry_path = file_system::month_folder(path, date);
    let entry_name = file_system::create_entry_name(prefix, date);
    entry_path.push(entry_name);
    return OpenOptions::new().append(true).open(entry_path);
}

fn add_content(
    file_result: io::Result<File>,
    content: String,
    tag: Option<&str>,
) -> Result<(), DiaryError> {
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

pub fn add(opts: &AddOptions, config: &Config, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let file_result = get_entry(config.diary_path().to_path_buf(), date, config.prefix());

    let content = edit::edit("")?;

    add_content(file_result, content, opts.tag)
}
