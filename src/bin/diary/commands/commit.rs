use std::str::FromStr;

use chrono::{Local, NaiveDate, ParseError, TimeZone};
use clap::{App, Arg, ArgMatches};
use diary::{
    config::ConfigManager,
    ops::commit::{commit, CommitOptions},
    CliResult, Diary,
};

pub fn cli() -> App<'static> {
    App::new("commit")
        .about("Commit an entry to git repo. Defaults to today's.")
        .arg(
            Arg::new("date")
                .long("date")
                .short('d')
                .value_name("date")
                .help("Commit a specific diary entry. Use the %Y-%m-%d format."),
        )
        .arg(
            Arg::new("message")
                .long("message")
                .short('m')
                .value_name("message")
                .help("Commit message to use."),
        )
        .arg(
            Arg::new("push")
                .long("push")
                .short('p')
                .required(false)
                .takes_value(false)
                .help("Whether or not to immediately push the commit."),
        )
}

fn args_to_commit_opts(args: &ArgMatches) -> Result<CommitOptions, ParseError> {
    let entry_date = match args.value_of("date") {
        Some(val) => {
            let date = NaiveDate::from_str(val)?;
            Local.from_utc_date(&date)
        }
        _ => Local::today(),
    };
    let message = args.value_of("message").unwrap_or("Added an entry.");
    let push = args.is_present("push");
    Ok(CommitOptions {
        entry_date,
        message: message.to_string(),
        push,
    })
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches) -> CliResult {
    let opts = args_to_commit_opts(args)?;
    let diary = Diary::from_config(config_manager.config())?;
    commit(&opts, &diary)?;
    println!("Committed entry."); // uncovered
    Ok(()) // uncovered
}
