mod cli;
mod commands;

fn main() {
    if let Err(error) = cli::main() {
        println!("{}", error.error.unwrap());
        std::process::exit(error.exit_code);
    }
}
