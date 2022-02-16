use std::str::FromStr;

use chrono::{Local, NaiveDate, ParseError, TimeZone};
use clap::{Arg, ArgMatches, Command};
use diary::{
    config::ConfigManager,
    ops::open::{open, OpenFileOptions},
    CliResult, Diary,
};

pub fn cli() -> Command<'static> {
    Command::new("open")
        .about("Open a specific diary entry. Defaults to today's.")
        .arg(
            Arg::new("date")
                .long("date")
                .short('d')
                .value_name("date")
                .help("Open a specific diary entry. Use the %Y-%m-%d format."),
        )
}

fn args_to_open_opts(args: &ArgMatches) -> Result<OpenFileOptions, ParseError> {
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
    let opts = args_to_open_opts(args)?;
    let diary = Diary::from_config(config_manager.config())?;
    open(&opts, &diary, edit::edit_file)?;
    println!("Opened diary entry."); // uncovered
    Ok(()) // uncovered
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::{Local, NaiveDate, TimeZone};

    use super::{args_to_open_opts, cli};

    #[test]
    fn test_date_provided() {
        let app = cli();

        let matches = app
            .try_get_matches_from(vec!["open", "--date", "2022-01-01"])
            .unwrap();

        let open_options = args_to_open_opts(&matches).unwrap();

        assert!(
            open_options.entry_date
                == Local.from_utc_date(&NaiveDate::from_str("2022-01-01").unwrap())
        )
    }
}
