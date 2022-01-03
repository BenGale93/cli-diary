#![doc = include_str!("../../README.md")]
extern crate confy;

#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod config;
pub mod entry;
pub mod errors;
pub mod ops;
pub mod utils;

pub type CliResult = Result<(), errors::CliError>;
