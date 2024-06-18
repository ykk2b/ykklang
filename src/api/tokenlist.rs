use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Colon,
    Comma,
    Dot,
    Plus,
    Minus,
    Asteric,
    Slash,
    Percent,
    Bang,
    Equal,
    More,
    Less,
    And,
    Pipe,
    Hash,
    At,
    Ask,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    ColonEqual,
    BangEqual,
    EqualEqual,
    MoreEqual,
    LessEqual,
    Arrow,
    Anon,
    AndShort,
    Or,
    // ExactEqual,
    // ...
    If,
    ElseIf,
    Else,
    From,
    Return,
    Module,
    Public,
    EoF,
    // ...
    NumberValue,
    StringValue,
    NullValue,
    VoidValue,
    BooleanValue,
    TrueValue,
    FalseValue,
    Identifier,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f32),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub token: Token,
    pub lexeme: String,
    pub value: Option<Value>,
    pub line_number: usize,
}

pub fn keywords() -> HashMap<&'static str, Token> {
    HashMap::from([
        // ...
        ("if", Token::If),
        ("else if", Token::Else),
        ("else", Token::Else),
        ("from", Token::From),
        ("return", Token::Return),
        ("module", Token::Module),
        ("public", Token::Public),
        ("anon", Token::Anon),
        // ...
        ("boolean", Token::BooleanValue),
        ("true", Token::TrueValue),
        ("false", Token::FalseValue),
        ("null", Token::NullValue),
        ("void", Token::VoidValue),
        ("string", Token::StringValue),
        ("number", Token::NumberValue),
    ])
}
