mod api;
mod backend;
mod frontend;
use std::process::exit;

use frontend::frontend;
use utils::cli::CLI;
mod utils;

fn main() {
    let cli = CLI::new();
    let input = match CLI::input(&cli) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            exit(1);
        }
    };
    frontend(&input);
}
