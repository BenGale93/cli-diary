use chrono::ParseError;
use clap::{Arg, ArgMatches, Command};
use diary::{
    config::ConfigManager,
    ops::open::{open, OpenFileOptions},
    utils::date::parse_date_option,
    CliResult, Diary,
};

pub fn cli() -> Command {
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
    let entry_date = parse_date_option(args)?;
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
                == Local.from_utc_datetime(
                    &NaiveDate::from_ymd_opt(2022, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                )
        )
    }
}
