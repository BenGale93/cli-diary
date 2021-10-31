extern crate clap;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, SubCommand};
use diary::{ops, CliResult, Config};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("init")
        .about("Create a new diary folder and config file.")
        .arg(Arg::with_name("path").default_value("."))
}

fn args_to_init_ops(args: &ArgMatches<'_>) -> ops::InitOptions {
    ops::InitOptions {
        path: PathBuf::from(args.value_of("path").unwrap()),
    }
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_init_ops(args);
    ops::init(opts, &config)?;
    println!("Ran init command");
    Ok(())
}
