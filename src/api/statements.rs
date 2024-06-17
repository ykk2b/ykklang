use super::{expressions::Expression, tokenlist::Unit};

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement {
        expression: Expression,
    },
    BlockStatement {
        statements: Vec<Box<Statement>>,
    },
    VariableStatement {
        name: Unit,
        value_type: Unit,
        value: Expression,
    },
    FunctionStatement {
        name: Unit,
        parameters: Vec<(Unit, Unit)>,
        value_type: Unit,
        body: Vec<Box<Statement>>,
    },
    ReturnStatement {
        value: Option<Expression>,
    },
    IfStatement {
        condition: Expression,
        body: Box<Statement>,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    BreakStatement {},
    ModStatement {
        name: Unit,
        from: Unit,
    },
    PublishStatement {
        name: Unit,
        statement: Box<Statement>,
    },
}
