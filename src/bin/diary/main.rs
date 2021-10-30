mod cli;
mod commands;

fn main() {
    if let Err(code) = cli::main() {
        std::process::exit(code.exit_code);
    }
}
