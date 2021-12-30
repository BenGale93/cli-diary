use std::{
    fs::File,
    io::{self, Write},
};

use crate::errors::DiaryError;

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
