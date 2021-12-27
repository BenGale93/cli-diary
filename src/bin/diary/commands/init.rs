extern crate clap;
use std::fs::canonicalize;
use std::path::PathBuf;

use clap::{App, Arg, ArgMatches, Error, ErrorKind, SubCommand};
use diary::ops::{init, InitOptions};
use diary::{CliResult, Config};

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

pub fn exec(config: Config, args: &ArgMatches<'_>) -> CliResult {
    let opts = args_to_init_ops(args)?;
    let path = init::init(&opts, &config)?;

    let new_prefix = match opts.prefix {
        Some(prefix) => prefix,
        None => config.prefix().to_string(),
    };

    let new_cfg = Config::builder()
        .diary_path(
            canonicalize(path).expect("Attempted to canonicalize a path that does not exist."),
        )
        .prefix(new_prefix)
        .build();
    confy::store("diary", new_cfg)?;
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
