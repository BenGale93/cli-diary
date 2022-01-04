use std::str::FromStr;

use chrono::{Local, NaiveDate, ParseError, TimeZone};
use clap::{App, Arg, ArgMatches};
use diary::{
    config::ConfigManager,
    ops::open::{open, OpenFileOptions},
    CliResult,
};

pub fn cli() -> App<'static> {
    App::new("open")
        .about("Open a specific diary entry. Defaults to today's.")
        .arg(
            Arg::new("date")
                .long("date")
                .short('d')
                .value_name("date")
                .help("Open a specific diary entry. Use the %Y-%m-%d format."),
        )
}

fn args_to_add_opts(args: &ArgMatches) -> Result<OpenFileOptions, ParseError> {
    let entry_date = match args.value_of("date") {
        Some(val) => {
            let date = NaiveDate::from_str(val)?;
            Local.from_utc_date(&date)
        }
        _ => Local::today(),
    };
    Ok(OpenFileOptions { entry_date })
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches) -> CliResult {
    let opts = args_to_add_opts(args)?;
    open(&opts, config_manager.config(), edit::edit_file)?;
    println!("Opened diary entry.");
    Ok(())
}
