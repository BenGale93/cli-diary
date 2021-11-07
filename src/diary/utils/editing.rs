use std::{fs::File, io::Write};

use chrono::prelude::*;
use edit;

use crate::{errors::DiaryError, utils::date};

pub fn user_edit_file(file: &mut File) -> Result<(), DiaryError> {
    let mut edited = edit::edit("")?;
    edited.push('\n');
    file.write_all(edited.as_bytes())?;
    Ok(())
}

pub fn add_title(file: &mut File, date: &DateTime<Local>) -> Result<(), DiaryError> {
    let full_title = format_title(date);
    file.write_all(full_title.as_bytes())?;
    Ok(())
}

fn format_title(date: &DateTime<Local>) -> String {
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
        let date = Local.ymd(2020, 10, 10).and_hms(0, 0, 0);

        let title = format_title(&date);

        assert_eq!(title, "# Saturday 10<sup>th</sup> October 2020\n\n");
    }
}
