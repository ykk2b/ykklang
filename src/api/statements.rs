use super::{expressions::Expression, tokenlist::Unit};

#[derive(Clone)]
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
        is_public: bool,
    },
    FunctionStatement {
        name: Unit,
        parameters: Vec<(Unit, Unit)>,
        value_type: Unit,
        body: Vec<Box<Statement>>,
        is_public: bool,
    },
    ReturnStatement {
        value: Expression,
    },
    IfStatement {
        condition: Expression,
        body: Box<Statement>,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    ModuleStatement {
        name: Unit,
        from: Unit,
    }
}
