extern crate clap;
use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use diary::{
    ops::new::{new, NewOptions},
    CliResult, Config,
};

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

fn args_to_new_opts(args: &ArgMatches<'_>) -> NewOptions {
    let open = args.is_present("open");
    NewOptions { open }
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_new_opts(args);
    let date = Local::now();
    new(&opts, &config, &date, edit::edit)?;
    println!("Ran new command");
    Ok(())
}
