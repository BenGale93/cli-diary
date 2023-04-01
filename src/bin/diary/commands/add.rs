use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use diary::{
    config::ConfigManager,
    ops::add::{add, AddOptions},
    CliResult, Diary,
};

pub fn cli() -> Command {
    Command::new("add")
        .about("Add a new sub-entry to today's diary.")
        .arg(
            Arg::new("tag")
                .long("tag")
                .short('t')
                .value_name("TAG")
                .help("Add a tag above the entry text."),
        )
}

fn args_to_add_opts(args: &ArgMatches) -> AddOptions {
    let tag = args.get_one::<String>("tag").cloned();
    AddOptions { tag }
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches) -> CliResult {
    let opts = args_to_add_opts(args);
    let date = Local::now();
    let diary = Diary::from_config(config_manager.config())?;
    add(&opts, &diary, &date, edit::edit)?;
    println!("Added content."); //uncovered.
    Ok(()) //uncovered.
}
