extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use diary::{CliResult, Config};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("init")
        .about("Create a new diary folder and config file.")
        .arg(Arg::with_name("path").default_value("."))
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    println!("Ran init command with {:?}", args);
    Ok(())
}
