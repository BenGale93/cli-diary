use clap::{App, ArgMatches};
use diary::Config;

pub fn builtin() -> Vec<App<'static, 'static>> {
    vec![init::cli()]
}

pub fn builtin_exec(cmd: &str) -> Option<fn(Config, &ArgMatches<'_>) -> diary::CliResult> {
    let f = match cmd {
        "init" => init::exec,
        _ => return None,
    };
    Some(f)
}

pub mod init;
