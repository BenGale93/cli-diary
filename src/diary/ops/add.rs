use std::{fs::OpenOptions, io::Write};

use chrono::{DateTime, Local};

use crate::{
    errors::DiaryError,
    utils::{editing, file_system},
    Config,
};

pub struct AddOptions<'a> {
    pub tag: Option<&'a str>,
}

pub fn add(opts: &AddOptions, config: &Config, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let mut entry_path = file_system::month_folder(config.diary_path().to_path_buf(), date);
    let entry_name = file_system::create_entry_name(config.prefix(), date);

    entry_path.push(entry_name);

    let result = OpenOptions::new().append(true).open(entry_path);

    match result {
        Ok(mut file) => {
            if let Some(tag) = opts.tag {
                let markdown_tag = format!("## {}\n\n", tag);
                file.write_all(markdown_tag.as_bytes())?;
            }
            editing::user_edit_file(&mut file)?;
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(DiaryError::NoEntry { source: e })
        }
        Err(e) => Err(e.into()),
    }
}
