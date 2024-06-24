use crate::api::tokenlist::Token::{self, *};
use crate::api::tokenlist::{keywords, Unit, Value};
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    input: String,
    units: Vec<Unit>,
    keywords: HashMap<&'static str, Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            units: vec![],
            keywords: keywords(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Unit>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.collect_tokens().expect("failed to collect tokens.")
        }
        self.units.push(Unit {
            token: EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: self.line,
        });
        Ok(self.units.clone())
    }

    fn collect_tokens(&mut self) -> Result<(), String> {
        let character = self.advance();
        match character {
            ':' => self.push_unit(Colon),
            ',' => self.push_unit(Comma),
            '.' => self.push_unit(Dot),
            '+' => self.push_unit(Plus),
            '-' => {
                if self.match_char('>') {
                    self.push_unit(Arrow)
                } else {
                    self.push_unit(Minus)
                }
            }
            '*' => self.push_unit(Multiplication),
            '/' => self.push_unit(Division),
            '%' => self.push_unit(Percent),
            '=' => {
                if self.match_char('=') {
                    self.push_unit(EqualEqual)
                } else {
                    self.push_unit(Equal)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.push_unit(GreaterEqual)
                } else {
                    self.push_unit(Greater)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.push_unit(LessEqual)
                } else {
                    self.push_unit(Less)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.push_unit(BangEqual)
                } else {
                    self.push_unit(Bang)
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.push_unit(And)
                } else {
                    self.push_unit(AndShort)
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.push_unit(Or)
                } else {
                    self.push_unit(Pipe)
                }
            }
            '#' => self.push_unit(Hash),
            '@' => self.push_unit(At),
            '?' => self.push_unit(QuestionMark),
            '{' => self.push_unit(LeftBrace),
            '}' => self.push_unit(RightBrace),
            '(' => self.push_unit(LeftParen),
            ')' => self.push_unit(RightParen),
            '[' => self.push_unit(LeftBracket),
            ']' => self.push_unit(RightBracket),
            ';' => self.push_unit(Semicolon),
            '"' => self.parse_string(),
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            _ => {
                if character.is_ascii_digit() {
                    self.parse_number().expect("failed to parse a number.");
                } else if character.is_alphabetic() || character == '_' || self.peek() == '!' {
                    self.parse_identifier();
                } else {
                    eprintln!("unexpected character (line {}): {}", self.line, character);
                }
            }
        }
        Ok(())
    }

    fn push_unit(&mut self, token: Token) {
        self.push_token_value(token, None)
    }

    fn push_token_value(&mut self, token: Token, value: Option<Value>) {
        let lexeme = self.input[self.start..self.current].to_string();
        self.units.push(Unit {
            token,
            lexeme,
            line_number: self.line,
            value,
        })
    }

    fn advance(&mut self) -> char {
        let character = self.input.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        character
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.current + 1).unwrap_or('\0')
        }
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("please terminate the string.");
            exit(1);
        }

        self.advance();
        let value = &self.input[self.start + 1..self.current - 1];
        self.push_token_value(StringValue, Some(Value::String(value.to_string())));
    }

    fn parse_number(&mut self) -> Result<(), String> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let substring = &self.input[self.start..self.current];
        let value = substring.parse::<f32>().expect("failed to parse a number");
        self.push_token_value(NumberValue, Some(Value::Number(value)));
        Ok(())
    }

    fn parse_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' || self.peek() == '!' {
            self.advance();
        }

        let substring = &self.input[self.start..self.current];
        let token = self.keywords.get(substring).cloned().unwrap_or(Identifier);
        self.push_unit(token);
    }
}
