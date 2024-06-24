use crate::api::Expression;
use crate::api::tokenlist::Token::{self, *};
use crate::api::types::ValueType;
use crate::api::{Statement, tokenlist::Unit};
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
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        if self.match_types() | self.match_token(Token::Public) {
            self.declaration()
        } else if self.match_token(LeftBrace) {
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
    fn module_statement(&mut self) -> Statement {
        let name = self.consume(Identifier, "expected module name");
        self.consume(From, "expected 'from' after the module name");
        let from = self.consume(StringValue, "expected module destination");
        self.consume(Semicolon, "expected ';' after value");
        Statement::Module { name, from }
    }
    fn return_statement(&mut self) -> Statement {
        self.previous(1);
        let value = self.expression();
        self.consume(Semicolon, "Expected ';' after return value;");
        Statement::Return { value }
    }
    fn if_statement(&mut self) -> Statement {
        let condition = self.expression();
        let body = Box::new(self.statement());
        let mut else_if_branches = Vec::new();

        while self.match_token(ElseIf) {
            let mut elif_predicates = Vec::new();
            loop {
                let elif_predicate = self.expression();
                elif_predicates.push(elif_predicate);
                if !self.match_token(Comma) {
                    break;
                }
            }
            let elif_stmt = Box::new(self.statement());
            else_if_branches.push((elif_predicates, elif_stmt));
        }

        let else_branch = if self.match_token(Else) {
            Some(Box::new(self.statement()))
        } else {
            None
        };

        Statement::If {
            condition,
            body,
            else_if_branches,
            else_branch,
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();
        self.consume(Semicolon, "expected ';' after an expression");
        Statement::Expression { expression }
    }

    fn declaration(&mut self) -> Statement {
        let mut is_public = false;
        let value_type;

        if self.previous(1).token == Token::Public {
            self.advance();
            is_public = true;
            value_type = self.previous(1);
        } else {
            value_type = self.previous(1);
        }
        let name = self.consume(Identifier, "expected a variable name");
        let mut parameters: Vec<(Unit, Unit)> = vec![];
        self.consume(LeftParen, "expected '(' after variable name");
        if !self.check(RightParen) {
            loop {
                let paramater_type = if self.match_types() {
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
        if self.match_token(Arrow) {
            let body = self.expression();
            self.consume(Semicolon, "expected ';' after an expression");
            return Statement::Function {
                name,
                parameters,
                value_type,
                is_public,
                body: vec![Statement::Return { value: body }],
            };
        }
        self.consume(LeftBrace, "expected '{' before function body");

        let body = match self.block_statement() {
            Statement::Block { statements } => statements,
            _ => {
                eprintln!(
                    "failed to parse a block statement at line {}",
                    name.line_number
                );
                exit(1);
            }
        };
        Statement::Function {
            name,
            parameters,
            value_type,
            body,
            is_public,
        }
    }

    fn block_statement(&mut self) -> Statement {
        let mut statements = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let declaration = self.statement();
            statements.push(declaration);
        }
        self.consume(RightBrace, "expected '}' after a block");
        Statement::Block { statements }
    }

    fn expression(&mut self) -> Expression {
        self.binary()
    }

    fn binary(&mut self) -> Expression {
        let mut expr: Expression = self.unary();
        while self.match_tokens(&[
            Or,
            Greater,
            GreaterEqual,
            Less,
            LessEqual,
            And,
            BangEqual,
            EqualEqual,
            Minus,
            Plus,
            Division,
            Multiplication,
            Percent,
        ]) {
            let op = self.previous(1);
            let rhs = self.unary();
            expr = Expression::Binary {
                id: self.get_id(),
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous(1);
            let rhs = self.unary();
            Expression::Unary {
                id: self.get_id(),
                left: Box::from(rhs),
                operator: op,
            }
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary();
        loop {
            if self.match_token(LeftParen) {
                expr = self
                    .finish_call(expr)
                    .expect("failed to parse an expression");
            } else {
                break;
            }
        }
        expr
    }

    fn primary(&mut self) -> Expression {
        let token = self.peek();
        let result;
        match token.token {
            Identifier => {
                self.advance();
                let mut expr = Expression::Variable {
                    id: self.get_id(),
                    name: self.previous(1),
                };

                if self.match_token(LeftBracket) {
                    let mut items = Vec::new();
                    while self.check(RightBracket) && self.is_at_end() {
                        let key = self.previous(1).lexeme.clone();

                        self.consume(Colon, "expected ':' after key");
                        let value = self.expression();

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

                    expr = Expression::Map {
                        id: self.get_id(),
                        items,
                    };
                }

                result = expr;
            }
            LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(RightParen, "expected ')' after an group");
                result = Expression::Grouping {
                    id: self.get_id(),
                    expression: Box::new(expr),
                };
            }
            FalseValue | TrueValue | NullValue | NumberValue | StringValue | VoidValue
            | BooleanValue => {
                self.advance();
                result = Expression::Value {
                    id: self.get_id(),
                    value: ValueType::from_unit(token),
                };
            }
            LeftBracket => {
                return self.parse_map();
            }
            _ => {
                eprintln!(
                    "unexpected token ('{}') at line {}",
                    token.lexeme, token.line_number
                );
                exit(1);
            }
        }
        result
    }

    fn parse_map(&mut self) -> Expression {
        let mut items = Vec::new();
        self.advance();
        while self.check(RightBracket) && self.is_at_end() {
            let key = self.previous(1).lexeme.clone();

            self.consume(Colon, "expected ':' after key");
            let value = self.expression();

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

        Expression::Map {
            id: self.get_id(),
            items,
        }
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut arguments = vec![];
        if !self.check(RightParen) {
            loop {
                let arg = self.expression();
                arguments.push(arg);
                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        self.consume(RightParen, "expected ')' after arguments.");
        Ok(Expression::Call {
            id: self.get_id(),
            name: Box::new(callee),
            arguments,
        })
    }

    fn consume(&mut self, token: Token, msg: &str) -> Unit {
        let unit: Unit = self.peek();
        if unit.token == token {
            self.advance();
            return self.previous(1);
        }

        eprintln!("{}, at line {}", msg, unit.line_number);
        unit.clone()
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

    fn match_types(&mut self) -> bool {
        self.match_tokens(&[
            NumberValue,
            StringValue,
            BooleanValue,
            TrueValue,
            FalseValue,
            NullValue,
            VoidValue,
        ])
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
