use clap::{App, ArgMatches};
use diary::config::ConfigManager;

pub fn builtin() -> Vec<App<'static>> {
    vec![init::cli(), new::cli(), add::cli(), open::cli()]
}

pub mod add;
pub mod init;
pub mod new;
pub mod open;

pub fn builtin_exec(cmd: &str) -> Option<fn(ConfigManager, &ArgMatches) -> diary::CliResult> {
    let f = match cmd {
        "init" => init::exec,
        "new" => new::exec,
        "add" => add::exec,
        "open" => open::exec,
        _ => return None,
    };
    Some(f)
}

#[cfg(test)]
mod test {
    use super::builtin_exec;

    #[test]
    fn test_bad_command() {
        let cmd_func = builtin_exec("fake");

        assert!(cmd_func.is_none())
    }
}
