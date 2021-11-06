use std::{
    fs::{create_dir, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::{errors::DiaryError, utils, Config};
use chrono::prelude::*;
use edit;

pub struct NewOptions {
    pub open: bool,
}

fn add_title(file: &mut File, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let start_title = date.format("%A %-e").to_string();
    let date_superscript = utils::date::date_superscript(date.day());
    let end_title = date.format("%B %Y").to_string();

    let full_title = format!(
        "# {}<sup>{}</sup> {}\n\n",
        start_title, date_superscript, end_title
    );

    file.write_all(full_title.as_bytes())?;
    Ok(())
}

pub fn new(opts: &NewOptions, config: &Config) -> Result<(), DiaryError> {
    let today = Local::now();
    let month_folder = PathBuf::from(today.format("%Y-%m").to_string());
    let entry_suffix = today.format("%Y-%m-%d").to_string();

    let mut new_entry_path: PathBuf = [config.diary_path(), &month_folder].iter().collect();

    if !new_entry_path.exists() {
        match create_dir(&new_entry_path) {
            Ok(_) => (),
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(DiaryError {
                    desc: String::from("Diary hasn't been initialised."),
                });
            }
            Err(e) => panic!("Unexpected IO error {}", e),
        }
    }

    let entry_name = PathBuf::from(format!("{}_{}.md", config.prefix(), entry_suffix));
    new_entry_path.push(entry_name);
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(new_entry_path);
    let mut file = match result {
        Ok(mut file) => {
            add_title(&mut file, &today)?;
            file
        }
        Err(e) => return Err(e.into()),
    };
    if opts.open {
        let edited = edit::edit("")?;
        file.write_all(edited.as_bytes())?;
    };
    Ok(())
}
