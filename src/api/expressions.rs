use super::{tokenlist::Unit, types::ValueType};

#[derive(Debug, Clone)]
pub enum Expression {
    ArrayExpression {
        id: usize,
        items: Vec<Box<Expression>>,
    },
    MapExpression {
        id: usize,
        items: Vec<(String, Box<Expression>)>,
    },
    VariableExpression {
        id: usize,
        name: Unit,
    },
    FunctionCallExpression {
        id: usize,
        name: Box<Expression>,
        arguments: Vec<Expression>,
    },
    UnaryLeftExpression {
        id: usize,
        left: Box<Expression>,
        operator: Unit,
    },
    UnaryRightExpression {
        id: usize,
        operator: Unit,
        right: Box<Expression>,
    },
    BinaryExpression {
        id: usize,
        left: Box<Expression>,
        operator: Unit,
        right: Box<Expression>,
    },
    GroupingExpression {
        id: usize,
        expression: Box<Expression>,
    },
    ValueExpression {
        id: usize,
        value: ValueType,
    },
}
