mod api;
mod backend;
mod frontend;
use frontend::frontend;
use utils::cli::CLI;
mod utils;

fn main() {
    let cli = CLI::new();
    let run = CLI::run(&cli);

    if run.is_some() {
        println!("running: {}", run.expect("failed to parse a 'run' command"))
    }

    frontend("let x: number = 10;");
    println!("Hello, world!");
}
