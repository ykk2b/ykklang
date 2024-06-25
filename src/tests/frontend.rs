#[allow(unused_imports)]
use crate::api::types::ValueType;
#[allow(unused_imports)]
use crate::{
    api::tokenlist::{Token, Unit, Value},
    frontend::tokenizer::Tokenizer,
};
#[allow(unused_imports)]
use crate::{
    api::{Expression, Statement},
    frontend::parser::Parser,
};

#[allow(dead_code)]
fn get_tokens(input: &str) -> Vec<Unit> {
    let tokenizer = Tokenizer::new(input.to_string());
    tokenizer.clone().tokenize().unwrap()
}

#[allow(dead_code)]
fn get_statements(tokens: Vec<Unit>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn function_one_param() {
    let tokens = get_tokens("null myfn(null a){return a;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);

    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![(
            Unit {
                token: Token::Identifier,
                lexeme: "a".to_string(),
                value: None,
                line_number: 1,
            },
            Unit {
                token: Token::NullValue,
                lexeme: "null".to_string(),
                value: None,
                line_number: 1,
            },
        )],
        value_type: Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Variable {
                id: 0,
                name: Unit {
                    token: Token::Identifier,
                    lexeme: "a".to_string(),
                    value: None,
                    line_number: 1,
                },
            },
        }],
        is_public: false,
    }];
    assert_eq!(statements, expected_statements);
}

#[test]
fn function_one_param_short() {
    let tokens = get_tokens("null myfn(null a) -> a;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![(
            Unit {
                token: Token::Identifier,
                lexeme: "a".to_string(),
                value: None,
                line_number: 1,
            },
            Unit {
                token: Token::NullValue,
                lexeme: "null".to_string(),
                value: None,
                line_number: 1,
            },
        )],
        value_type: Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Variable {
                id: 0,
                name: Unit {
                    token: Token::Identifier,
                    lexeme: "a".to_string(),
                    value: None,
                    line_number: 1,
                },
            },
        }],
        is_public: false,
    }];
    assert_eq!(statements, expected_statements);
}

#[test]
fn function_two_params() {
    let tokens = get_tokens("number myfn(number a, number b){return a + b;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Comma,
            lexeme: ",".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "b".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Plus,
            lexeme: "+".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "b".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![
            (
                Unit {
                    token: Token::Identifier,
                    lexeme: "a".to_string(),
                    value: None,
                    line_number: 1,
                },
                Unit {
                    token: Token::NumberValue,
                    lexeme: "number".to_string(),
                    value: None,
                    line_number: 1,
                },
            ),
            (
                Unit {
                    token: Token::Identifier,
                    lexeme: "b".to_string(),
                    value: None,
                    line_number: 1,
                },
                Unit {
                    token: Token::NumberValue,
                    lexeme: "number".to_string(),
                    value: None,
                    line_number: 1,
                },
            ),
        ],
        value_type: Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Binary {
                id: 2,
                left: Box::new(Expression::Variable {
                    id: 0,
                    name: Unit {
                        token: Token::Identifier,
                        lexeme: "a".to_string(),
                        value: None,
                        line_number: 1,
                    },
                }),
                operator: Unit {
                    token: Token::Plus,
                    lexeme: "+".to_string(),
                    value: None,
                    line_number: 1,
                },
                right: Box::new(Expression::Variable {
                    id: 1,
                    name: Unit {
                        token: Token::Identifier,
                        lexeme: "b".to_string(),
                        value: None,
                        line_number: 1,
                    },
                }),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_two_params_short() {
    let tokens = get_tokens("number myfn(number a, number b) -> a + b;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Comma,
            lexeme: ",".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "b".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "a".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Plus,
            lexeme: "+".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "b".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![
            (
                Unit {
                    token: Token::Identifier,
                    lexeme: "a".to_string(),
                    value: None,
                    line_number: 1,
                },
                Unit {
                    token: Token::NumberValue,
                    lexeme: "number".to_string(),
                    value: None,
                    line_number: 1,
                },
            ),
            (
                Unit {
                    token: Token::Identifier,
                    lexeme: "b".to_string(),
                    value: None,
                    line_number: 1,
                },
                Unit {
                    token: Token::NumberValue,
                    lexeme: "number".to_string(),
                    value: None,
                    line_number: 1,
                },
            ),
        ],
        value_type: Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Binary {
                id: 2,
                left: Box::new(Expression::Variable {
                    id: 0,
                    name: Unit {
                        token: Token::Identifier,
                        lexeme: "a".to_string(),
                        value: None,
                        line_number: 1,
                    },
                }),
                operator: Unit {
                    token: Token::Plus,
                    lexeme: "+".to_string(),
                    value: None,
                    line_number: 1,
                },
                right: Box::new(Expression::Variable {
                    id: 1,
                    name: Unit {
                        token: Token::Identifier,
                        lexeme: "b".to_string(),
                        value: None,
                        line_number: 1,
                    },
                }),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_void_no_param() {
    let tokens = get_tokens("void myfn(){}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::VoidValue,
            lexeme: "void".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::VoidValue,
            lexeme: "void".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_null_no_param() {
    let tokens = get_tokens("null myfn(){return null;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::Null,
            },
        }],
        is_public: false,
    }];
    assert_eq!(statements, expected_statements);
}

#[test]
fn function_null_no_param_short() {
    let tokens = get_tokens("null myfn() -> null;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::NullValue,
            lexeme: "null".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::Null,
            },
        }],
        is_public: false,
    }];
    assert_eq!(statements, expected_statements);
}

#[test]
fn function_number_no_param() {
    let tokens = get_tokens("number myfn(){return 5;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "5".to_string(),
            value: Some(Value::Number(5.0)),
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::Number(5.0),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_number_no_param_short() {
    let tokens = get_tokens("number myfn() -> 5;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "5".to_string(),
            value: Some(Value::Number(5.0)),
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::NumberValue,
            lexeme: "number".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::Number(5.0),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_string_no_param() {
    let tokens = get_tokens("string myfn(){return \"string\";}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::StringValue,
            lexeme: "string".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"string\"".to_string(),
            value: Some(Value::String("string".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::StringValue,
            lexeme: "string".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::String("string".to_string()),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_string_no_param_short() {
    let tokens = get_tokens("string myfn() -> \"string\";");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::StringValue,
            lexeme: "string".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"string\"".to_string(),
            value: Some(Value::String("string".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::StringValue,
            lexeme: "string".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::String("string".to_string()),
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

//

#[test]
fn function_boolean_no_param() {
    let tokens = get_tokens("boolean myfn(){return true;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::BooleanValue,
            lexeme: "boolean".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::BooleanValue,
            lexeme: "boolean".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::True,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_boolean_no_param_short() {
    let tokens = get_tokens("boolean myfn() -> false;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::BooleanValue,
            lexeme: "boolean".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::BooleanValue,
            lexeme: "boolean".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::False,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

//

#[test]
fn function_true_no_param() {
    let tokens = get_tokens("true myfn(){return true;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::True,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_true_no_param_short() {
    let tokens = get_tokens("true myfn() -> true;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::True,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

//

#[test]
fn function_false_no_param() {
    let tokens = get_tokens("false myfn(){return false;}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::False,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_false_no_param_short() {
    let tokens = get_tokens("false myfn() -> false;");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);

    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Value {
                id: 0,
                value: ValueType::False,
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_map_no_param() {
    let tokens = get_tokens("map myfn(){return [\"key\": \"value\"];}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::MapValue,
            lexeme: "map".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Return,
            lexeme: "return".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBracket,
            lexeme: "[".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"key\"".to_string(),
            value: Some(Value::String("key".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::Colon,
            lexeme: ":".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"value\"".to_string(),
            value: Some(Value::String("value".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::RightBracket,
            lexeme: "]".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::MapValue,
            lexeme: "map".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Map {
                id: 1,
                items: vec![(
                    "\"key\"".to_string(),
                    ValueType::String("value".to_string()),
                )],
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_map_no_param_short() {
    let tokens = get_tokens("map myfn() -> [\"key\": \"value\"];");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::MapValue,
            lexeme: "map".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Arrow,
            lexeme: "->".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBracket,
            lexeme: "[".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"key\"".to_string(),
            value: Some(Value::String("key".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::Colon,
            lexeme: ":".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::StringValue,
            lexeme: "\"value\"".to_string(),
            value: Some(Value::String("value".to_string())),
            line_number: 1,
        },
        Unit {
            token: Token::RightBracket,
            lexeme: "]".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
    let statements = get_statements(tokens.clone());
    let expected_statements: Vec<Statement> = vec![Statement::Function {
        name: Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        parameters: vec![],
        value_type: Unit {
            token: Token::MapValue,
            lexeme: "map".to_string(),
            value: None,
            line_number: 1,
        },
        body: vec![Statement::Return {
            value: Expression::Map {
                id: 1,
                items: vec![(
                    "\"key\"".to_string(),
                    ValueType::String("value".to_string()),
                )],
            },
        }],
        is_public: false,
    }];

    assert_eq!(statements, expected_statements);
}

#[test]
fn function_call_no_param() {
    let tokens = get_tokens("myfn();");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn function_call_one_param() {
    let tokens = get_tokens("myfn(5);");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "5".to_string(),
            value: Some(Value::Number(5.0)),
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn function_call_two_params() {
    let tokens = get_tokens("myfn(5, true);");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::Identifier,
            lexeme: "myfn".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftParen,
            lexeme: "(".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::NumberValue,
            lexeme: "5".to_string(),
            value: Some(Value::Number(5.0)),
            line_number: 1,
        },
        Unit {
            token: Token::Comma,
            lexeme: ",".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: true.to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightParen,
            lexeme: ")".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Semicolon,
            lexeme: ";".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn if_statement() {
    let tokens = get_tokens("if true {}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn if_else_statement() {
    let tokens = get_tokens("if true {} else {}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Else,
            lexeme: "else".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn if_else_if_statement() {
    let tokens = get_tokens("if true {} else if false {}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Else,
            lexeme: "else".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn if_else_if_else_statement() {
    let tokens = get_tokens("if true {} else if false {} else {}");
    let expected_tokens: Vec<Unit> = vec![
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::TrueValue,
            lexeme: "true".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Else,
            lexeme: "else".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::If,
            lexeme: "if".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::FalseValue,
            lexeme: "false".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::Else,
            lexeme: "else".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::LeftBrace,
            lexeme: "{".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::RightBrace,
            lexeme: "}".to_string(),
            value: None,
            line_number: 1,
        },
        Unit {
            token: Token::EoF,
            lexeme: "".to_string(),
            value: None,
            line_number: 1,
        },
    ];

    assert_eq!(tokens, expected_tokens);
}
