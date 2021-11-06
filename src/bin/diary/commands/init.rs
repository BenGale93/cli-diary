extern crate clap;
use std::fs::canonicalize;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use diary::errors::DiaryError;
use diary::ops::init;
use diary::{CliResult, Config};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("init")
        .about("Create a new diary folder and config file.")
        .arg(Arg::with_name("path").default_value("."))
}

fn args_to_init_ops(args: &ArgMatches<'_>) -> Result<init::InitOptions, DiaryError> {
    if let Some(path) = args.value_of("path") {
        Ok(init::InitOptions {
            path: PathBuf::from(path),
        })
    } else {
        Err(DiaryError {
            desc: String::from("Value for path argument not found."),
        })
    }
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_init_ops(args)?;
    let path = init::init(&opts, &config)?;
    let new_cfg = Config::new(
        canonicalize(path).expect("Attempted to canonicalize a path that does not exist."),
        config.prefix().to_string(),
    );
    confy::store("diary", new_cfg)?;
    println!("Ran init command");
    Ok(())
}
