use crate::api::{tokenlist::Unit, types::Module, Expression, Statement};
use std::{collections::HashMap, process::exit};

#[derive(Copy, Clone, PartialEq, Debug)]
enum IsFunction {
    Yes,
    No,
}

#[derive(Debug, Clone)]
pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    locals: HashMap<usize, usize>,
    current_function: IsFunction,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            locals: HashMap::new(),
            current_function: IsFunction::No,
        }
    }
    pub fn resolve(
        &mut self,
        statements: &Vec<&Statement>,
        module: &mut Module,
    ) -> HashMap<usize, usize> {
        self.resolve_many(statements, module);
        self.locals.clone()
    }
    fn resolve_internal(&mut self, statement: &Statement, module: &mut Module) {
        match statement {
            Statement::Block { statements: _ } => self.resolve_block(statement, module),
            Statement::Expression { expression } => self.resolve_expression(expression),
            Statement::Function {
                name: _,
                parameters: _,
                value_type: _statements,
                body: _,
                is_public: _,
            } => self.resolve_function(statement, IsFunction::Yes, module),
            Statement::If {
                condition: _,
                body: _,
                else_if_branches: _,
                else_branch: _,
            } => self.resolve_if_statement(statement, module),
            Statement::Module { name: _, from: _ } => {}
            Statement::Return { value } => {
                if self.current_function == IsFunction::No {
                    eprintln!("you can't use return outside the function");
                } else {
                    self.resolve_expression(value);
                }
            }
        }
    }
    fn resolve_block(&mut self, statement: &Statement, module: &mut Module) {
        match statement {
            Statement::Block { statements } => {
                self.start_scope();
                self.resolve_many(&statements.iter().collect(), module)
            }
            _ => {
                eprintln!("failed to resolve a block statement");
                exit(1)
            }
        }
    }
    fn resolve_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expression(left);
                self.resolve_expression(right)
            }
            Expression::Call {
                id: _,
                name,
                arguments,
            } => {
                self.resolve_expression(name.as_ref());
                for argument in arguments {
                    self.resolve_expression(argument)
                }
            }
            Expression::Grouping { id: _, expression } => {
                self.resolve_expression(expression);
            }
            Expression::Map { id: _, items: _ } => {}
            Expression::Unary {
                id: _,
                left,
                operator: _,
            } => self.resolve_expression(left),
            Expression::Variable { id: _, name: _ } => {
                let id = expression.get_id();
                match expression {
                    Expression::Variable { id: _, name } => {
                        if !self.scopes.is_empty() {
                            if let Some(false) =
                                self.scopes[self.scopes.len() - 1].get(&name.lexeme)
                            {
                                eprintln!("failed to read a local variable");
                                exit(1);
                            }
                        }
                        self.resolve_local(name, id)
                    }
                    Expression::Call {
                        id: _,
                        name,
                        arguments: _,
                    } => match name.as_ref() {
                        Expression::Variable { id: _, name } => self.resolve_local(name, id),
                        _ => {
                            eprintln!("failed to resolve a variable statement");
                            exit(1);
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn resolve_function(
        &mut self,
        statement: &Statement,
        function: IsFunction,
        module: &mut Module,
    ) {
        if let Statement::Function {
            name: _,
            parameters,
            value_type: _,
            body,
            is_public: _,
        } = statement
        {
            let enclosing_function = self.current_function;
            self.current_function = function;
            self.start_scope();
            for (parameter_name, _parameter_type) in parameters {
                self.declare(parameter_name);
                self.define(parameter_name);
            }
            self.resolve_many(&body.iter().collect(), module);
            self.end_scope();
            self.current_function = enclosing_function;
        } else {
            eprintln!("failed to resolve non-function statement");
            exit(1);
        }
    }
    fn resolve_if_statement(&mut self, statement: &Statement, module: &mut Module) {
        if let Statement::If {
            condition,
            body,
            else_if_branches,
            else_branch,
        } = statement
        {
            self.resolve_expression(condition);
            self.resolve_internal(body.as_ref(), module);
            for (else_if_predicates, else_if_statement) in else_if_branches {
                for else_if_predicate in else_if_predicates {
                    self.resolve_expression(else_if_predicate)
                }
                self.resolve_internal(else_if_statement.as_ref(), module)
            }
            if let Some(branch) = else_branch {
                self.resolve_internal(branch, module);
            }
        } else {
            eprintln!("failed to resolve an if statement");
            exit(1);
        }
    }

    fn start_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop().expect("stack underflow");
    }
    fn declare(&mut self, name: &Unit) {
        let size = self.scopes.len();
        if self.scopes.is_empty() {
            return;
        } else if self.scopes[size - 1].contains_key(&name.lexeme.clone()) {
            eprintln!("'{}' is already declared", name.lexeme.clone());
            exit(1)
        }
        self.scopes[size - 1].insert(name.lexeme.clone(), false);
    }
    fn define(&mut self, name: &Unit) {
        if self.scopes.is_empty() {
            return;
        }
        let size = self.scopes.len();
        self.scopes[size - 1].insert(name.lexeme.clone(), true);
    }
    fn resolve_local(&mut self, name: &Unit, id: usize) {
        let size = self.scopes.len();
        if size == 0 {
            return;
        }
        for i in (0..=(size - 1)).rev() {
            let scope = &self.scopes[i];
            if scope.contains_key(&name.lexeme) {
                self.locals.insert(id, size - i - 1);
                return;
            }
        }
    }
    fn resolve_many(&mut self, statements: &Vec<&Statement>, module: &mut Module) {
        for statement in statements {
            self.resolve_internal(statement, module);
        }
    }
}
