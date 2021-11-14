use chrono::{Local, NaiveDate, ParseError, TimeZone};
use clap::{App, Arg, ArgMatches, SubCommand};
use diary::ops::open::open;
use diary::ops::open::OpenFileOptions;
use diary::{CliResult, Config};
use std::str::FromStr;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("open")
        .about("Open a specific diary entry. Defaults to today's.")
        .arg(
            Arg::with_name("date")
                .long("date")
                .short("d")
                .value_name("date")
                .help("Open a specific diary entry. Use the %Y-%m-%d format."),
        )
}

fn args_to_add_opts(args: &ArgMatches<'_>) -> Result<OpenFileOptions, ParseError> {
    let entry_date = match args.value_of("date") {
        Some(val) => {
            let date = NaiveDate::from_str(val)?;
            Local.from_utc_date(&date)
        }
        _ => Local::today(),
    };
    Ok(OpenFileOptions { entry_date })
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_add_opts(args)?;
    open(&opts, &config, edit::edit_file)?;
    println!("Ran open command");
    Ok(())
}
