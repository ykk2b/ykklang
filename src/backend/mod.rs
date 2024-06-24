use interpreter::Interpreter;
use resolver::Resolver;

use crate::api::Statement;

pub mod expressions;
pub mod interpreter;
pub mod modules;
pub mod resolver;

pub fn backend(statements: Vec<Statement>, mut interpreter: Interpreter) {
    let mut resolver = Resolver::new();
    let locals = resolver.resolve(&statements.iter().collect(), &mut interpreter.module);
    interpreter.resolve(locals);
    interpreter.interpret(statements.iter().collect());
}
