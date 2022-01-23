extern crate clap;
use chrono::Local;
use clap::{App, Arg, ArgMatches};
use diary::{
    config::ConfigManager,
    ops::new::{new, NewOptions},
    CliResult, Diary,
};

pub fn cli() -> App<'static> {
    App::new("new")
        .about("Create a new diary entry for today.")
        .arg(
            Arg::new("open")
                .long("open")
                .short('o')
                .help("Open the new entry after creation."),
        )
}

fn args_to_new_opts(args: &ArgMatches) -> NewOptions {
    let open = args.is_present("open");
    NewOptions { open }
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches) -> CliResult {
    let opts = args_to_new_opts(args);
    let date = Local::today();
    let diary = Diary::from_config(config_manager.config())?;
    new(&opts, &diary, &date, edit::edit)?;
    println!("Created today's entry.");
    Ok(())
}
