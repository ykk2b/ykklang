use crate::api::{Expression, Statement};

pub struct Optimizer {
    statements: Vec<Statement>,
}

impl Optimizer {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn optimize(&mut self) -> Vec<Statement> {
        let mut functions_called: Vec<String> = Vec::new();
        let mut new_statements: Vec<Statement> = Vec::new();

        for statement in &self.statements {
            if let Statement::Expression { expression } = statement {
                if let Expression::Call { name, .. } = expression {
                    if let Expression::Variable { name, .. } = name.as_ref() {
                        functions_called.push(name.lexeme.clone());
                    }
                }
            }
        }

        for statement in &self.statements {
            match statement {
                Statement::Function { name, is_public, .. } => {
                    if *is_public || functions_called.contains(&name.lexeme) {
                        new_statements.push(statement.clone());
                    }
                }
                _ => {
                    new_statements.push(statement.clone());
                }
            }
        }

        self.statements = new_statements.clone();
        new_statements
    }
}
