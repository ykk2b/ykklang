use tokenlist::Unit;
use types::ValueType;
pub mod tokenlist;
pub mod types;

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Map {
        id: usize,
        items: Vec<(String, ValueType)>,
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
    Unary {
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

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Expression {
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
    Function {
        name: Unit,
        parameters: Vec<(Unit, Unit)>,
        value_type: Unit,
        body: Vec<Statement>,
        is_public: bool,
    },
    Return {
        value: Expression,
    },
    If {
        condition: Expression,
        body: Box<Statement>,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    Module {
        name: Unit,
        from: Unit,
    },
}
