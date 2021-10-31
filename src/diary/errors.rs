use std::{error, fmt};

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

#[derive(Debug)]
pub struct DiaryError {
    pub desc: String,
}

impl fmt::Display for DiaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.desc.fmt(f)
    }
}

impl error::Error for DiaryError {}
