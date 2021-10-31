use clap::{App, ArgMatches};
use diary::{CliResult, Config};

use crate::commands;

pub fn main() -> CliResult {
    let args = match cli().get_matches_safe() {
        Ok(args) => args,
        Err(e) => e.exit(),
    };

    let (cmd, subcommand_args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            cli().print_help()?;
            return Ok(());
        }
    };

    let config = confy::load("diary")?;
    execute_subcommand(config, cmd, subcommand_args)
}

fn cli() -> App<'static, 'static> {
    App::new("diary").subcommands(commands::builtin())
}

fn execute_subcommand(config: Config, cmd: &str, subcommand_args: &ArgMatches<'_>) -> CliResult {
    let exec = commands::builtin_exec(cmd);
    exec.unwrap()(config, subcommand_args)
}
