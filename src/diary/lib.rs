extern crate confy;

#[macro_use]
extern crate serde_derive;

extern crate serde;

mod config;
mod errors;

pub type CliResult = Result<(), errors::CliError>;

pub use config::Config;
