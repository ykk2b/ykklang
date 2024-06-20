mod api;
mod backend;
mod frontend;
use std::process::exit;

use backend::{backend, interpreter::Interpreter};
use frontend::frontend;
use utils::cli::Cli;
mod utils;

fn main() {
    let cli = Cli::new();
    let input = match Cli::input(&cli) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            exit(1);
        }
    };
    let interpreter = Interpreter::new();
    let statements = frontend(&input);
    backend(statements, interpreter);
}
