use std::io;

use chrono::ParseError;
use thiserror::Error;

pub struct CliError {
    pub error: Option<anyhow::Error>,
    pub exit_code: i32,
}

impl CliError {
    pub fn new(error: anyhow::Error, code: i32) -> CliError {
        CliError {
            error: Some(error),
            exit_code: code,
        }
    }

    pub fn code(code: i32) -> CliError {
        CliError {
            error: None,
            exit_code: code,
        }
    }
}

impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> CliError {
        CliError::new(err, 101)
    }
}

impl From<clap::Error> for CliError {
    fn from(err: clap::Error) -> CliError {
        let code = if err.use_stderr() { 1 } else { 0 };
        CliError::new(err.into(), code)
    }
}

impl From<confy::ConfyError> for CliError {
    fn from(err: confy::ConfyError) -> CliError {
        CliError::new(err.into(), 101)
    }
}
impl From<DiaryError> for CliError {
    fn from(err: DiaryError) -> CliError {
        CliError::new(err.into(), 202)
    }
}

impl From<ParseError> for CliError {
    fn from(err: ParseError) -> CliError {
        CliError::new(err.into(), 101)
    }
}
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::new(err.into(), 1)
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

    #[error("Today's entry has not yet been created. Use the `new` sub-command.")]
    NoEntry { source: Option<std::io::Error> },

    #[error("No content provided, aborting.")]
    NoContent,

    #[error("Failed to init git repo.")]
    GitError(#[from] git2::Error),
}
