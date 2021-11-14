use std::{
    fs::File,
    io::{self, Write},
};

use chrono::prelude::*;

use crate::{errors::DiaryError, utils::date};

pub type StringGetter = fn(S: String) -> io::Result<String>;

pub fn test_string_getter(template: String) -> io::Result<String> {
    let output = template + "Test content";
    Ok(output)
}

pub fn test_empty_string_getter(_template: String) -> io::Result<String> {
    Ok("".to_string())
}
pub fn add_user_content_to_file(file: &mut File, mut content: String) -> Result<(), DiaryError> {
    content.push('\n');
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn add_title(file: &mut File, date: &Date<Local>) -> Result<(), DiaryError> {
    let full_title = format_title(date);
    file.write_all(full_title.as_bytes())?;
    Ok(())
}

fn format_title(date: &Date<Local>) -> String {
    let start_title = date.format("%A %-e").to_string();
    let date_superscript = date::date_superscript(date.day());
    let end_title = date.format("%B %Y").to_string();

    format!(
        "# {}<sup>{}</sup> {}\n\n",
        start_title, date_superscript, end_title
    )
}

#[cfg(test)]
mod tests {

    use super::format_title;
    use chrono::prelude::*;

    #[test]
    fn test_format_title() {
        let date = Local.ymd(2020, 10, 10);

        let title = format_title(&date);

        assert_eq!(title, "# Saturday 10<sup>th</sup> October 2020\n\n");
    }
}
