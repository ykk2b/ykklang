use crate::api::tokenlist::Unit;
use crate::api::Statement;
use optimizer::Optimizer;
use parser::Parser;
use tokenizer::Tokenizer;
pub mod optimizer;
pub mod parser;
pub mod tokenizer;

pub fn frontend(input: &str) -> Vec<Statement> {
    let tokenizer = Tokenizer::new(input.to_string());
    let tokens: Vec<Unit> = tokenizer.clone().tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    let mut optimizer = Optimizer::new(statements);
    optimizer.optimize()
}
