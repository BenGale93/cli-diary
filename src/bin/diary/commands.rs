use clap::{App, ArgMatches};
use diary::config::ConfigManager;

pub fn builtin() -> Vec<App<'static, 'static>> {
    vec![init::cli(), new::cli(), add::cli(), open::cli()]
}

pub mod add;
pub mod init;
pub mod new;
pub mod open;

pub fn builtin_exec(cmd: &str) -> Option<fn(ConfigManager, &ArgMatches<'_>) -> diary::CliResult> {
    let f = match cmd {
        "init" => init::exec,
        "new" => new::exec,
        "add" => add::exec,
        "open" => open::exec,
        _ => return None,
    };
    Some(f)
}
