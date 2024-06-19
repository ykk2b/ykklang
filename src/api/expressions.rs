use super::{statements::Statement, tokenlist::Unit, types::ValueType};

#[derive(Clone)]
pub enum Expression {
    MapExpression {
        id: usize,
        items: Vec<(String, Box<Expression>)>,
    },
    VariableExpression {
        id: usize,
        name: Unit,
    },
    AnonymousFunctionExpression {
        id: usize,
        parameters: Vec<(Unit, Unit)>,
        value_type: Unit,
        body: Vec<Box<Statement>>,
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
