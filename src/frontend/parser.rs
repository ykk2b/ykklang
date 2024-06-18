use crate::api::expressions::Expression;
use crate::api::tokenlist::Token::{self, *};
use crate::api::types::ValueType;
use crate::api::{statements::Statement, tokenlist::Unit};
use std::process::exit;

pub struct Parser {
    units: Vec<Unit>,
    current: usize,
    next_id: usize,
}

impl Parser {
    pub fn new(units: Vec<Unit>) -> Self {
        Self {
            units,
            current: 0,
            next_id: 0,
        }
    }
    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];
        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(s) => statements.push(s),
                Err(_) => {
                    eprintln!("failed to parse a statement.");
                    exit(1);
                }
            }
        }
        Ok(statements)
    }
    fn declaration(&mut self) -> Result<Statement, String> {
        if self.match_token(Let) {
            self.var_declaration()
        } else if self.match_token(Function) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }
    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_token(LeftBrace) {
            self.block_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(Module) {
            self.module_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }
    fn module_statement(&mut self) -> Result<Statement, String> {
        let name = self.consume(Identifier, "expected module name");
        self.consume(From, "expected 'from' after the module name");
        let from = self.consume(StringValue, "expected module destination");
        self.consume(Semicolon, "expected ';' after value");
        Ok(Statement::ModuleStatement { name, from })
    }
    fn return_statement(&mut self) -> Result<Statement, String> {
        self.previous(1);
        let value;
        if !self.check(Semicolon) {
            value = Some(self.expression().expect("failed to parse an expression"));
        } else {
            value = None;
        }
        self.consume(Semicolon, "Expected ';' after return value;");
        Ok(Statement::ReturnStatement { value })
    }
    fn if_statement(&mut self) -> Result<Statement, String> {
        let condition = self.expression().expect("failed to parse an expression");
        let body = Box::new(self.statement().expect("failed to parse a statement"));
        let mut else_if_branches = Vec::new();

        while self.match_token(ElseIf) {
            let mut elif_predicates = Vec::new();
            loop {
                let elif_predicate = self.expression().expect("failed to parse an expression");
                elif_predicates.push(elif_predicate);
                if !self.match_token(Comma) {
                    break;
                }
            }
            let elif_stmt = Box::new(self.statement().expect("failed to parse a statement"));
            else_if_branches.push((elif_predicates, elif_stmt));
        }

        let else_branch = if self.match_token(Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::IfStatement {
            condition,
            body,
            else_if_branches,
            else_branch,
        })
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expression = self.expression().expect("failed to parse an expression");
        self.consume(Semicolon, "expected ';' after expression");
        Ok(Statement::ExpressionStatement { expression })
    }

    fn var_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume(Identifier, "expected a variable name");
        self.consume(Colon, "expected ':' after a variable name");
        let value_type = if self.match_tokens(&[
            StringValue,
            NumberValue,
            VoidValue,
            NullValue,
            BooleanValue,
            TrueValue,
            FalseValue,
        ]) {
            self.previous(1)
        } else {
            eprintln!("expected type after ':' at line {}", name.line_number);
            exit(1);
        };
        self.consume(Equal, "expected '=' after a variable name");
        let value = self.expression().expect("failed to parse an expression");
        self.consume(Semicolon, "expected ';' after variable declaration");
        Ok(Statement::VariableStatement {
            name,
            value_type,
            value,
        })
    }

    fn function_declaration(&mut self) -> Result<Statement, String> {
        let name = self.consume(Identifier, "expected function name");
        self.consume(LeftParen, "expected '(' after a function name");
        let mut parameters: Vec<(Unit, Unit)> = vec![];

        if !self.check(RightParen) {
            if parameters.len() >= 32 {
                eprintln!(
                    "function can't have more then 32 parameters, at line {}",
                    name.line_number
                );
                exit(1);
            }
            loop {
                let paramater_type = if self.match_tokens(&[
                    StringValue,
                    NumberValue,
                    VoidValue,
                    NullValue,
                    BooleanValue,
                    TrueValue,
                    FalseValue,
                ]) {
                    self.previous(1)
                } else {
                    eprintln!("expected parameter type at line {}", name.line_number);
                    exit(1);
                };

                let paramater_name = self.consume(Identifier, "expected parameter name");

                parameters.push((paramater_name, paramater_type));
                if !self.match_token(Comma) {
                    break;
                }
            }
        }

        self.consume(RightParen, "expected ')' after parameters");
        self.consume(Arrow, "expected '->' after parameters");
        let value_type = if self.match_tokens(&[
            StringValue,
            NumberValue,
            VoidValue,
            NullValue,
            BooleanValue,
            TrueValue,
            FalseValue,
        ]) {
            self.previous(1)
        } else {
            eprintln!("expected type after '->' at line {}", name.line_number);
            exit(1);
        };
        self.consume(LeftBrace, "expected '{' before function body");

        let body = match self
            .block_statement()
            .expect("failed to parse a block statement")
        {
            Statement::BlockStatement { statements } => statements,
            _ => {
                eprintln!(
                    "failed to parse a block statement at line {}",
                    name.line_number
                );
                exit(1);
            }
        };
        Ok(Statement::FunctionStatement {
            name,
            parameters,
            value_type,
            body,
        })
    }

    fn block_statement(&mut self) -> Result<Statement, String> {
        let mut statements = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let declaration = self.declaration().expect("failed to parse a declaration");
            statements.push(Box::new(declaration));
        }
        self.consume(RightBrace, "expected '}' after a block");
        Ok(Statement::BlockStatement { statements })
    }

    fn expression(&mut self) -> Result<Expression, String> {
        let expr = self.or().expect("failed to parse an expression");

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, String> {
        let mut expr = self.and().expect("failed to parse an expression");
        while self.match_token(Pipe) {
            let operator = self.previous(1);
            let right = self.and().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, String> {
        let mut expr = self.equality().expect("failed to parse an expression");
        while self.match_token(And) {
            let operator = self.previous(1);
            let right = self.equality().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.comparasion().expect("failed to parse an expression");
        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous(1);
            let rhs = self.comparasion().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn comparasion(&mut self) -> Result<Expression, String> {
        let mut expr = self.term().expect("failed to parse an expression");
        while self.match_tokens(&[More, MoreEqual, Less, LessEqual]) {
            let op = self.previous(1);
            let rhs = self.term().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor().expect("failed to parse an expression");
        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous(1);
            let rhs = self.factor().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary_right().expect("failed to parse an expression");
        while self.match_tokens(&[Slash, Asteric]) {
            let op = self.previous(1);
            let rhs = self.unary_right().expect("failed to parse an expression");
            expr = Expression::BinaryExpression {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        Ok(expr)
    }

    fn unary_right(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Percent]) {
            let op = self.previous(1);
            let rhs = self.unary_right().expect("failed to parse an expression");
            Ok(Expression::UnaryRightExpression {
                id: self.get_id(),
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.unary_left()
        }
    }

    fn unary_left(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous(1);
            let rhs = self.unary_left().expect("failed to parse an expression");
            Ok(Expression::UnaryLeftExpression {
                id: self.get_id(),
                left: Box::from(rhs),
                operator: op,
            })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary().expect("failed to parse an expression");
        loop {
            if self.match_token(LeftParen) {
                expr = self
                    .finish_call(expr)
                    .expect("failed to parse an expression");
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression, String> {
        let token = self.peek();
        let result;
        match token.token {
            Identifier => {
                self.advance();
                let mut expr = Expression::VariableExpression {
                    id: self.get_id(),
                    name: self.previous(1),
                };

                if self.match_token(LeftBracket) {
                    let mut items = Vec::new();
                    while self.check(RightBracket) && self.is_at_end() {
                        let key = self.previous(1).lexeme.clone();

                        self.consume(Colon, "expected ':' after key");
                        let value = self.expression().expect("failed to parse a value");

                        if items
                            .iter()
                            .any(|item: &(String, Box<Expression>)| item.0 == key)
                        {
                            eprintln!("key '{:?}' already exists in the map.", key);
                            exit(1);
                        }

                        items.push((key, Box::new(value)));

                        if self.match_token(Comma) {
                            break;
                        }
                    }
                    self.consume(RightBracket, "Expected ']' after map elements.");

                    expr = Expression::MapExpression {
                        id: self.get_id(),
                        items,
                    };
                } else if self.match_token(LeftParen) {
                    let mut parameters: Vec<(Unit, Unit)> = vec![];

                    if !self.check(RightParen) {
                        if parameters.len() >= 32 {
                            eprintln!(
                                "function can't have more then 32 parameters, at line {}",
                                self.previous(1).line_number
                            );
                            exit(1);
                        }
                        loop {
                            let paramater_type =
                                self.consume(Identifier, "expected parameter type");

                            let paramater_name =
                                self.consume(Identifier, "expected parameter name");

                            parameters.push((paramater_name, paramater_type));
                            if !self.match_token(Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(RightParen, "expected ')' after parameters");
                    self.consume(Arrow, "expected '->' after parameters");
                    let value_type = self.consume(Identifier, "expected type after '->'");
                    self.consume(LeftBrace, "expected '{' before function body");

                    let body = match self
                        .block_statement()
                        .expect("failed to parse a block statement")
                    {
                        Statement::BlockStatement { statements } => statements,
                        _ => {
                            eprintln!(
                                "failed to parse a block statement at line {}",
                                value_type.line_number
                            );
                            exit(1);
                        }
                    };

                    expr = Expression::AnonymousFunctionExpression {
                        id: self.get_id(),
                        parameters,
                        value_type,
                        body,
                    }
                }

                result = expr;
            }
            LeftParen => {
                self.advance();
                let expr = self.expression().expect("failed to parse an expression");
                self.consume(RightParen, "Expected ')' after expression");
                result = Expression::GroupingExpression {
                    id: self.get_id(),
                    expression: Box::new(expr),
                };
            }
            FalseValue | TrueValue | NullValue | NumberValue | StringValue | VoidValue
            | BooleanValue => {
                self.advance();
                result = Expression::ValueExpression {
                    id: self.get_id(),
                    value: ValueType::from_unit(token),
                };
            }
            LeftBracket => {
                return self.parse_map();
            }
            _ => {
                eprintln!("unexpected token at line {}", token.line_number);
                exit(1);
            }
        }
        Ok(result)
    }

    fn parse_map(&mut self) -> Result<Expression, String> {
        let mut items = Vec::new();
        self.advance();
        while self.check(RightBracket) && self.is_at_end() {
            let key = self.previous(1).lexeme.clone();

            self.consume(Colon, "expected ':' after key");
            let value = self.expression().expect("failed to parse a value");

            if items
                .iter()
                .any(|item: &(String, Box<Expression>)| item.0 == key)
            {
                eprintln!("key '{:?}' already exists in the map.", key);
                exit(1);
            }

            items.push((key, Box::new(value)));

            if self.match_token(Comma) {
                break;
            }
        }
        self.consume(RightBracket, "Expect ']' after array elements.");

        Ok(Expression::MapExpression {
            id: self.get_id(),
            items,
        })
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut arguments = vec![];
        if !self.check(RightParen) {
            loop {
                let arg = self.expression().expect("failed to parse an expression");
                arguments.push(arg);
                if arguments.len() >= 32 {
                    eprintln!(
                        "function can't have more then 32 arguments, at line {}",
                        self.previous(1).line_number
                    );
                } else if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "expected ')' after arguments.");
        Ok(Expression::FunctionCallExpression {
            id: self.get_id(),
            name: Box::new(callee),
            arguments,
        })
    }

    fn consume(&mut self, token: Token, msg: &str) -> Unit {
        let unit: Unit = self.peek();
        if unit.token == token {
            self.advance();
            let token = self.previous(1);
            return Ok::<Unit, String>(token).expect("failed to consume a character");
        }

        eprintln!("{}, at line {}", msg, unit.line_number);
        Ok::<Unit, String>(unit.clone())
            .expect(format!("failed to consume a character at line {}", unit.line_number).as_str())
    }

    fn check(&mut self, token: Token) -> bool {
        self.peek().token == token
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek().token == token {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.match_token(*token) {
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Unit {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous(1)
    }

    fn previous(&mut self, steps_back: usize) -> Unit {
        if self.current < steps_back {
            Unit {
                token: EoF,
                lexeme: String::new(),
                line_number: 0,
                value: None,
            }
        } else {
            self.units[self.current - steps_back].clone()
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token == EoF
    }

    fn peek(&mut self) -> Unit {
        self.units[self.current].clone()
    }
}
