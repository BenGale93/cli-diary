use crate::utils::date;
use crate::utils::file_system;
use chrono::prelude::*;
use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io,
};

pub trait DiaryFile {
    fn new(prefix: String, diary_path: PathBuf) -> Self;

    fn get_extension(&self) -> &'static str;

    fn get_prefix(&self) -> &String;
    fn get_diary_path(&self) -> PathBuf;

    fn title(&self, date: &Date<Local>) -> String;

    fn tag(&self, tag_name: String) -> String;

    fn file_name(&self, date: &Date<Local>) -> PathBuf {
        let entry_suffix = date.format("%Y-%m-%d").to_string();
        let file_name = format!(
            "{}_{}.{}",
            self.get_prefix(),
            entry_suffix,
            self.get_extension()
        );
        PathBuf::from(file_name)
    }
    fn get_entry_path(&self, date: &Date<Local>) -> PathBuf {
        let mut entry_path = file_system::month_folder(self.get_diary_path(), date);
        let entry_name = self.file_name(date);
        entry_path.push(entry_name);
        entry_path
    }
    fn get_entry(&self, date: &Date<Local>) -> io::Result<File> {
        let entry_path = self.get_entry_path(date);
        return OpenOptions::new().append(true).open(entry_path);
    }
}

pub struct MarkdownDiary {
    prefix: String,
    diary_path: PathBuf,
}

impl DiaryFile for MarkdownDiary {
    fn new(prefix: String, diary_path: PathBuf) -> Self {
        Self { prefix, diary_path }
    }

    fn get_prefix(&self) -> &String {
        &self.prefix
    }
    fn get_diary_path(&self) -> PathBuf {
        self.diary_path.to_path_buf()
    }
    fn get_extension(&self) -> &'static str {
        "md"
    }

    fn title(&self, date: &Date<Local>) -> String {
        let start_title = date.format("%A %-e").to_string();
        let date_superscript = date::date_superscript(date.day());
        let end_title = date.format("%B %Y").to_string();

        format!(
            "# {}<sup>{}</sup> {}\n\n",
            start_title, date_superscript, end_title
        )
    }

    fn tag(&self, tag_name: String) -> String {
        format!("## {}\n\n", tag_name)
    }
}
