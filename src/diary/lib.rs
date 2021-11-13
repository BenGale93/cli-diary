#![doc = include_str!("../../README.md")]
extern crate confy;

#[macro_use]
extern crate serde_derive;
extern crate serde;

mod config;
pub mod errors;
pub mod ops;
pub mod utils;
pub use config::Config;

pub type CliResult = Result<(), errors::CliError>;
