extern crate confy;

#[macro_use]
extern crate serde_derive;

extern crate serde;

mod config;
pub mod errors;

pub mod ops;
pub mod utils;

pub type CliResult = Result<(), errors::CliError>;

pub use config::Config;
