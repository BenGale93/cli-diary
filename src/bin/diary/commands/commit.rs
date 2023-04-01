use chrono::ParseError;
use clap::{Arg, ArgMatches, Command};
use diary::{
    config::ConfigManager,
    ops::commit::{commit, CommitOptions},
    utils::date::parse_date_option,
    CliResult, Diary,
};

pub fn cli() -> Command {
    Command::new("commit")
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
                .num_args(0)
                .help("Whether or not to immediately push the commit."),
        )
}

fn args_to_commit_opts(args: &ArgMatches) -> Result<CommitOptions, ParseError> {
    let entry_date = parse_date_option(args)?;
    let message = args
        .get_one::<String>("message")
        .cloned()
        .unwrap_or("Added an entry.".to_owned());
    let push = args.get_flag("push");
    Ok(CommitOptions {
        entry_date,
        message,
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
