use super::{statements::Statement, tokenlist::Unit, types::ValueType};

#[derive(Clone)]
pub enum Expression {
    Map {
        id: usize,
        items: Vec<(String, Box<Expression>)>,
    },
    Variable {
        id: usize,
        name: Unit,
    },
    Anonymous {
        id: usize,
        parameters: Vec<(Unit, Unit)>,
        value_type: Unit,
        body: Vec<Statement>,
    },
    Call {
        id: usize,
        name: Box<Expression>,
        arguments: Vec<Expression>,
    },
    UnaryLeft {
        id: usize,
        left: Box<Expression>,
        operator: Unit,
    },
    UnaryRight {
        id: usize,
        operator: Unit,
        right: Box<Expression>,
    },
    Binary {
        id: usize,
        left: Box<Expression>,
        operator: Unit,
        right: Box<Expression>,
    },
    Grouping {
        id: usize,
        expression: Box<Expression>,
    },
    Value {
        id: usize,
        value: ValueType,
    },
}
