extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use diary::{CliResult, Config};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("new")
        .about("Create a new diary entry for today.")
        .arg(
            Arg::with_name("open")
                .long("open")
                .short("o")
                .help("Open the new entry after creation."),
        )
}

pub fn exec(_config: Config, _args: &ArgMatches<'_>) -> CliResult {
    todo!()
}
