use std::{
    fs::{File, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use chrono::prelude::*;
use enum_dispatch::enum_dispatch;

use crate::{
    config::Config,
    errors::DiaryError,
    utils::{date, file_system},
};

fn title_elements(date: &Date<Local>) -> (String, String, String) {
    let start_title = date.format("%A %-e").to_string();
    let date_superscript = date::date_superscript(date.day()).to_string();
    let end_title = date.format("%B %Y").to_string();

    (start_title, date_superscript, end_title)
}

#[enum_dispatch]
pub trait EntryContent {
    fn extension(&self) -> &'static str;

    fn title(&self, date: &Date<Local>) -> String;

    fn tag(&self, tag_name: String) -> String;
}

pub struct MarkdownDiary {}

impl EntryContent for MarkdownDiary {
    fn extension(&self) -> &'static str {
        "md"
    }

    fn title(&self, date: &Date<Local>) -> String {
        let (start_title, date_superscript, end_title) = title_elements(date);

        format!(
            "# {}<sup>{}</sup> {}\n\n",
            start_title, date_superscript, end_title
        )
    }

    fn tag(&self, tag_name: String) -> String {
        format!("## {}\n\n", tag_name)
    }
}
pub struct RstDiary {}

impl EntryContent for RstDiary {
    fn extension(&self) -> &'static str {
        "rst"
    }

    fn title(&self, date: &Date<Local>) -> String {
        let (start_title, date_superscript, end_title) = title_elements(date);

        let first_line = format!(
            "{}\\ :sup:`{}` {}",
            start_title, date_superscript, end_title
        );
        let first_line_len = first_line.chars().count();

        let second_line = format!("{:=<1$}", "", first_line_len);

        format!("{}\n{}\n\n", first_line, second_line)
    }

    fn tag(&self, tag_name: String) -> String {
        let first_line_len = tag_name.chars().count();

        let second_line = format!("{:^<1$}", "", first_line_len);

        format!("{}\n{}\n\n", tag_name, second_line)
    }
}

#[enum_dispatch(EntryContent)]
pub enum EntryFileType {
    MarkdownDiary,
    RstDiary,
}

fn entry_file_type_from_string(file_type: impl AsRef<str>) -> Result<EntryFileType, DiaryError> {
    match file_type.as_ref() {
        "md" => Ok(MarkdownDiary {}.into()),
        "rst" => Ok(RstDiary {}.into()),
        _ => Err(DiaryError::BadFileType),
    }
}

pub fn process_file_type(
    potential_file_type: Option<impl AsRef<str>>,
) -> Result<Option<impl AsRef<str>>, DiaryError> {
    match potential_file_type {
        None => Ok(None),
        Some(file_type) => {
            let result = entry_file_type_from_string(&file_type);
            if result.is_err() {
                Err(DiaryError::BadFileType)
            } else {
                Ok(Some(file_type))
            }
        }
    }
}

pub struct Entry {
    prefix: String,
    diary_path: PathBuf,
    file_type: EntryFileType,
}

impl Entry {
    pub fn new(prefix: &str, diary_path: &Path, file_type: &str) -> Result<Box<Self>, DiaryError> {
        let entry_file_type = entry_file_type_from_string(file_type)?;
        Ok(Box::new(Self {
            prefix: prefix.to_string(),
            diary_path: diary_path.to_path_buf(),
            file_type: entry_file_type,
        }))
    }

    pub fn from_config(cfg: &Config) -> Result<Box<Self>, DiaryError> {
        Entry::new(cfg.prefix(), cfg.diary_path(), cfg.file_type())
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }
    pub fn diary_path(&self) -> PathBuf {
        self.diary_path.to_path_buf()
    }
    pub fn file_type(&self) -> &EntryFileType {
        &self.file_type
    }
    pub fn file_name(&self, date: &Date<Local>) -> PathBuf {
        let entry_suffix = date.format("%Y-%m-%d").to_string();
        let file_name = format!(
            "{}_{}.{}",
            self.prefix,
            entry_suffix,
            self.file_type.extension()
        );
        PathBuf::from(file_name)
    }
    pub fn get_entry_path(&self, date: &Date<Local>) -> PathBuf {
        let mut entry_path = file_system::month_folder(self.diary_path(), date);
        let entry_name = self.file_name(date);
        entry_path.push(entry_name);
        entry_path
    }
    pub fn get_entry(&self, date: &Date<Local>) -> io::Result<File> {
        let entry_path = self.get_entry_path(date);
        return OpenOptions::new().append(true).open(entry_path);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::prelude::*;

    use super::{
        entry_file_type_from_string, process_file_type, Entry, EntryContent, MarkdownDiary,
        RstDiary,
    };
    use crate::config::Config;

    #[test]

    fn get_extension() {
        let entry_file = entry_file_type_from_string("rst").unwrap();

        assert_eq!(entry_file.extension(), "rst");

        let entry_file = entry_file_type_from_string("md").unwrap();

        assert_eq!(entry_file.extension(), "md")
    }

    #[test]
    #[should_panic(expected = "value: BadFileType")]
    fn unsupported_file_extension() {
        process_file_type(Some("xyz")).unwrap();
    }

    #[test]
    fn rst_title() {
        let entry_file = RstDiary {};
        let entry_date = Local.ymd(2021, 11, 6);

        let actual_header = entry_file.title(&entry_date);

        let expected_header =
            "Saturday 6\\ :sup:`th` November 2021\n===================================\n\n";

        assert_eq!(actual_header, expected_header)
    }

    #[test]
    fn rst_tag() {
        let entry_file = RstDiary {};

        let actual_tag = entry_file.tag("Meeting".to_string());

        let expected_tag = "Meeting\n^^^^^^^\n\n";

        assert_eq!(actual_tag, expected_tag)
    }

    #[test]
    fn md_title() {
        let entry_file = MarkdownDiary {};
        let entry_date = Local.ymd(2021, 11, 6);

        let actual_header = entry_file.title(&entry_date);

        let expected_header = "# Saturday 6<sup>th</sup> November 2021\n\n";

        assert_eq!(actual_header, expected_header)
    }

    #[test]
    fn markdown_tag() {
        let entry_file = MarkdownDiary {};

        let actual_tag = entry_file.tag("Meeting".to_string());

        let expected_tag = "## Meeting\n\n";

        assert_eq!(actual_tag, expected_tag)
    }

    #[test]
    fn diary_file_from_config() {
        let cfg = Config::builder().file_type("md").build();

        let diary_file = Entry::from_config(&cfg).unwrap();

        assert_eq!(diary_file.file_type().extension(), "md");
        assert_eq!(diary_file.diary_path(), PathBuf::from(""));
        assert_eq!(diary_file.prefix(), "diary")
    }
}
