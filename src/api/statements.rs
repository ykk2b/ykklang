use super::{expressions::Expression, tokenlist::Unit};

#[derive(Clone)]
pub enum Statement {
    Expression {
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
    Variable{
        name: Unit,
        value_type: Unit,
        value: Expression,
        is_public: bool,
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
    If{
        condition: Expression,
        body: Box<Statement>,
        else_if_branches: Vec<(Vec<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    Module {
        name: Unit,
        from: Unit,
    }
}
