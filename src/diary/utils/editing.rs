use std::{
    fs::File,
    io::{self, Write},
};

use crate::errors::DiaryError;

pub type StringGetter = fn(S: String) -> io::Result<String>;

pub fn add_user_content_to_file(file: &mut File, content: String) -> Result<(), DiaryError> {
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
pub mod test {
    use std::io;

    pub fn test_string_getter(template: String) -> io::Result<String> {
        let output = template + "Test content";
        Ok(output)
    }

    pub fn test_empty_string_getter(_template: String) -> io::Result<String> {
        Ok("".to_string())
    }
}
