mod cli;
mod commands;

fn main() {
    if let Err(error) = cli::main() {
        if let Some(inner_error) = error.error {
            println!("{}", inner_error);
        }
        std::process::exit(error.exit_code);
    }
}
