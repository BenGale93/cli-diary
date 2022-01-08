use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};
use diary::{config, errors, CliResult};

use crate::commands;

pub fn main() -> CliResult {
    let args = match cli().try_get_matches() {
        Ok(args) => args,
        Err(e) => e.exit(),
    };

    let config_value = args.value_of("config").map(PathBuf::from);

    let config_manager = config::ConfigManager::with_location(config_value).read()?;

    let (cmd, subcommand_args) = match args.subcommand() {
        Some((cmd, args)) => (cmd, args),
        _ => {
            cli().print_help()?;
            return Ok(());
        }
    };

    execute_subcommand(config_manager, cmd, subcommand_args)
}

fn cli() -> App<'static> {
    App::new("diary")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommands(commands::builtin())
}

fn execute_subcommand(
    config_manager: config::ConfigManager,
    cmd: &str,
    subcommand_args: &ArgMatches,
) -> CliResult {
    let exec_opt = commands::builtin_exec(cmd);
    match exec_opt {
        Some(exec) => exec(config_manager, subcommand_args),
        None => Err(errors::CliError::code(1)),
    }
}
