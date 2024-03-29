use std::io;

use chrono::ParseError;
use thiserror::Error;

pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32,
}

impl CliError {
    pub fn new(error: anyhow::Error, code: i32) -> Self {
        Self {
            error: Some(error),
            exit_code: code,
        }
    }

    // uncovered.
    pub const fn code(code: i32) -> Self {
        Self {
            error: None,
            exit_code: code,
        }
    }
}

impl From<anyhow::Error> for CliError {
    // uncovered.
    fn from(err: anyhow::Error) -> Self {
        Self::new(err, 101)
    }
}

impl From<clap::Error> for CliError {
    // uncovered.
    fn from(err: clap::Error) -> Self {
        let code = if err.use_stderr() { 1 } else { 0 };
        Self::new(err.into(), code)
    }
}

impl From<confy::ConfyError> for CliError {
    // uncovered.
    fn from(err: confy::ConfyError) -> Self {
        Self::new(err.into(), 101)
    }
}
impl From<DiaryError> for CliError {
    // uncovered.
    fn from(err: DiaryError) -> Self {
        Self::new(err.into(), 202)
    }
}

impl From<ParseError> for CliError {
    fn from(err: ParseError) -> Self {
        Self::new(err.into(), 101)
    }
}
impl From<io::Error> for CliError {
    // uncovered.
    fn from(err: io::Error) -> Self {
        Self::new(err.into(), 1)
    }
}

#[derive(Error, Debug)]
pub enum DiaryError {
    #[error("Diary folder already exists somewhere.")]
    ExistsElsewhere,

    #[error("Diary folder already exists at the path provided.")]
    ExistsHere,

    #[error("Diary has not been initialised. Use the `init` sub-command.")]
    UnInitialised { source: Option<std::io::Error> },

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("The desired entry has not been found. You can use the `new` command to create today's entry.")]
    NoEntry { source: Option<std::io::Error> },

    #[error("No content provided, aborting.")]
    NoContent,

    #[error("Unsupported file type.")]
    BadFileType,

    #[error(transparent)]
    GitError(#[from] git2::Error),
}
