use crate::api::tokenlist::Unit;
use crate::api::Statement;
use crate::utils::cli::Cli;
use optimizer::Optimizer;
use parser::Parser;
use tokenizer::Tokenizer;
pub mod optimizer;
pub mod parser;
pub mod tokenizer;

pub fn frontend(input: &str, cli: Cli) -> Vec<Statement> {
    let tokenizer = Tokenizer::new(input.to_string());
    let tokens: Vec<Unit> = tokenizer.clone().tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    if cli.no_optimize() {
        return statements;
    }
    let mut optimizer = Optimizer::new(statements);
    optimizer.optimize()
}
