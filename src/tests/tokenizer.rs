#[allow(unused_imports)]
use crate::{
    api::tokenlist::{Token, Unit, Value},
    frontend::tokenizer::Tokenizer,
};

#[allow(dead_code)]
fn get_tokens(input: &str) -> Vec<Unit> {
    let tokenizer = Tokenizer::new(input.to_string());
    tokenizer.clone().tokenize().unwrap()
}

#[test]
fn function_one_param() {
    let tokens = get_tokens("null myfn(null a){return a;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_one_param_short() {
    let tokens = get_tokens("null myfn(null a) -> a;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_two_params() {
    let tokens = get_tokens("number myfn(number a, number b){return a + b;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_two_params_short() {
    let tokens = get_tokens("number myfn(number a, number b) -> a + b;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_void_no_param() {
    let tokens = get_tokens("void myfn(){}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_void_one_param() {
    let tokens = get_tokens("void myfn(number a){}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_null_no_param() {
    let tokens = get_tokens("null myfn(){return null;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_null_no_param_short() {
    let tokens = get_tokens("null myfn() -> null;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_number_no_param() {
    let tokens = get_tokens("number myfn(){return 5;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_number_no_param_short() {
    let tokens = get_tokens("number myfn() -> 5;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

//

#[test]
fn function_string_no_param() {
    let tokens = get_tokens("string myfn(){return \"string\";}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_string_no_param_short() {
    let tokens = get_tokens("string myfn() -> \"string\";");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

//

#[test]
fn function_boolean_no_param() {
    let tokens = get_tokens("boolean myfn(){return true;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_boolean_no_param_short() {
    let tokens = get_tokens("boolean myfn() -> false;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

//

#[test]
fn function_true_no_param() {
    let tokens = get_tokens("true myfn(){return true;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_true_no_param_short() {
    let tokens = get_tokens("true myfn() -> true;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

//

#[test]
fn function_false_no_param() {
    let tokens = get_tokens("false myfn(){return false;}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_false_no_param_short() {
    let tokens = get_tokens("false myfn() -> false;");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_call_no_param() {
    let tokens = get_tokens("myfn();");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_call_one_param() {
    let tokens = get_tokens("myfn(5);");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn function_call_two_params() {
    let tokens = get_tokens("myfn(5, true);");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn if_statement() {
    let tokens = get_tokens("if true {}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn if_else_statement() {
    let tokens = get_tokens("if true {} else {}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn if_else_if_statement() {
    let tokens = get_tokens("if true {} else if false {}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}

#[test]
fn if_else_if_else_statement() {
    let tokens = get_tokens("if true {} else if false {} else {}");
    let expected: Vec<Unit> = vec![
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

    assert_eq!(tokens, expected);
}
