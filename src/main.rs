mod api;
mod backend;
mod frontend;
mod tests;
use backend::{backend, interpreter::Interpreter};
use frontend::frontend;
use utils::cli::Cli;
mod utils;

fn main() {
    let cli = Cli::new();
    let input = Cli::run(&cli);
    let interpreter = Interpreter::new();
    let statements = frontend(&input, cli);
    backend(statements, interpreter);
}
