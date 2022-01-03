extern crate clap;
use std::{fs::canonicalize, path::PathBuf};

use clap::{App, Arg, ArgMatches, Error, ErrorKind, SubCommand};
use diary::{
    config::{Config, ConfigManager},
    entry::process_file_type,
    ops::{init, InitOptions},
    CliResult,
};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("init")
        .about("Create a new diary folder and config file.")
        .arg(
            Arg::with_name("path")
                .default_value(".")
                .help("The location you would like the diary folder to be created."),
        )
        .arg(
            Arg::with_name("repo")
                .long("repo")
                .short("r")
                .required(false)
                .takes_value(false)
                .help("Whether or not to initialise a git repo in the diary folder."),
        )
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .takes_value(true)
                .help("Sets the diary files name prefix."),
        )
        .arg(
            Arg::with_name("filetype")
                .long("filetype")
                .takes_value(true)
                .help("Sets the file type to use for diary entries."),
        )
}

fn args_to_init_ops(args: &ArgMatches<'_>) -> Result<init::InitOptions, Error> {
    let diary_path = match args.value_of("path") {
        Some(path) => PathBuf::from(path),
        None => {
            return Err(Error {
                message: String::from("Value for path argument not found."),
                kind: ErrorKind::MissingRequiredArgument,
                info: None,
            });
        }
    };
    let diary_prefix = args.value_of("prefix").map(String::from);

    let git_repo = args.is_present("repo");

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

pub fn exec(config_manager: ConfigManager, args: &ArgMatches<'_>) -> CliResult {
    let processed_file_type = process_file_type(args.value_of("filetype"))?;

    let opts = args_to_init_ops(args)?;
    let path = init::init(&opts, config_manager.config())?;

    let new_cfg = build_new_config(path, opts.prefix, processed_file_type);

    config_manager.update_config(new_cfg).write()?;

    println!("Initialised diary.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{args_to_init_ops, cli};

    #[test]
    fn test_with_prefix() {
        let app = cli();

        let args = app
            .clone()
            .get_matches_from(vec!["init", "--prefix", "test"]);

        let opts = args_to_init_ops(&args).unwrap();

        assert!(opts.prefix == Some(String::from("test")));
        assert!(opts.path == PathBuf::from("."));
        assert!(!opts.git_repo);
    }
}
