extern crate clap;
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
    let new_arg = args.value_of("open");
    let open = new_arg.is_some();
    NewOptions { open }
}

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_new_opts(args);
    new(&opts, &config)?;
    println!("Ran new command");
    Ok(())
}
