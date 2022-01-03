use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use diary::{
    config::ConfigManager,
    ops::add::{add, AddOptions},
    CliResult,
};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add a new sub-entry to today's diary.")
        .arg(
            Arg::with_name("tag")
                .long("tag")
                .short("t")
                .value_name("TAG")
                .help("Add a tag above the entry text."),
        )
}

fn args_to_add_opts<'a>(args: &'a ArgMatches<'_>) -> AddOptions<'a> {
    let tag = args.value_of("tag");
    AddOptions { tag }
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_add_opts(args);
    let date = Local::today();
    add(&opts, config_manager.config(), &date, edit::edit)?;
    println!("Added content.");
    Ok(())
}
