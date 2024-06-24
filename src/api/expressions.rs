use super::{tokenlist::Unit, types::ValueType};

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Map {
        id: usize,
        items: Vec<(String, Box<Expression>)>,
    },
    Variable {
        id: usize,
        name: Unit,
    },
    Call {
        id: usize,
        name: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Unary{
        id: usize,
        left: Box<Expression>,
        operator: Unit,
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
