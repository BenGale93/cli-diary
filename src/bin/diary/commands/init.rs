extern crate clap;
use std::{fs::canonicalize, path::PathBuf};

use clap::{error::ErrorKind, Arg, ArgMatches, Command, Error};
use diary::{
    config::{Config, ConfigManager},
    ops::{init, InitOptions},
    process_file_type, CliResult,
};

pub fn cli() -> Command {
    Command::new("init")
        .about("Create a new diary folder and config file.")
        .arg(
            Arg::new("path")
                .default_value(".")
                .help("The location you would like the diary folder to be created."),
        )
        .arg(
            Arg::new("repo")
                .long("repo")
                .short('r')
                .required(false)
                .num_args(0)
                .help("Whether or not to initialise a git repo in the diary folder."),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .num_args(1)
                .help("Sets the diary files name prefix."),
        )
        .arg(
            Arg::new("filetype")
                .long("filetype")
                .num_args(1)
                .help("Sets the file type to use for diary entries."),
        )
}

fn args_to_init_ops(args: &ArgMatches) -> Result<init::InitOptions, Error> {
    let diary_path = match args.get_one::<String>("path") {
        Some(path) => PathBuf::from(path),
        None => {
            return Err(Error::raw(
                ErrorKind::MissingRequiredArgument,
                "Value for path argument not found.",
            )); // uncovered.
        }
    };
    let diary_prefix = args.get_one::<String>("prefix").cloned();

    let git_repo = args.get_flag("repo");

    Ok(InitOptions {
        path: diary_path,
        prefix: diary_prefix,
        git_repo,
    })
}

fn build_new_config(
    diary_path: PathBuf,
    prefix: Option<String>,
    processed_file_type: Option<impl AsRef<str>>,
) -> Config {
    let new_cfg_builder = Config::builder().diary_path(
        canonicalize(diary_path).expect("Attempted to canonicalize a path that does not exist."),
    );

    let new_cfg_builder = match prefix {
        None => new_cfg_builder,
        Some(prefix) => new_cfg_builder.prefix(prefix),
    };

    match processed_file_type {
        None => new_cfg_builder,
        Some(file_type) => new_cfg_builder.file_type(file_type.as_ref()),
    }
    .build()
}

pub fn exec(config_manager: ConfigManager, args: &ArgMatches) -> CliResult {
    let processed_file_type =
        process_file_type(args.get_one::<String>("filetype").map(|x| x.as_str()))?;

    let opts = args_to_init_ops(args)?;
    let path = init::init(&opts, config_manager.config().diary_path())?;

    let new_cfg = build_new_config(path, opts.prefix, processed_file_type);

    config_manager.update_config(new_cfg).write()?;

    println!("Initialised diary.");
    Ok(())
}
