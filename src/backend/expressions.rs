use std::{
    hash::{Hash, Hasher},
    process::exit,
};

use crate::api::{
    expressions::Expression,
    tokenlist::{Token, Unit, Value},
    types::{FunctionValueType, Module, ValueType},
};

use super::interpreter::Interpreter;

impl ValueType {
    pub fn to_string(&self) -> String {
        match self {
            ValueType::Map(_) => "map".to_string(),
            ValueType::Boolean(bool) => bool.to_string(),
            ValueType::DeclaredFunction(fun) => format!("{}()", fun.name),
            ValueType::False => "false".to_string(),
            ValueType::True => "true".to_string(),
            ValueType::Null => "null".to_string(),
            ValueType::Void => "void".to_string(),
            ValueType::Number(_) => "number".to_string(),
            ValueType::String(_) => "string".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn from_unit(unit: Unit) -> Self {
        match unit.token {
            Token::NumberValue => {
                let number = match unit.value {
                    Some(Value::Number(x)) => x,
                    _ => {
                        eprintln!("failed to unwrap a number");
                        exit(1)
                    }
                };
                Self::Number(number)
            }
            Token::StringValue => {
                let string = match unit.value {
                    Some(Value::String(s)) => s.clone(),
                    _ => {
                        eprintln!("failed to unwrap a string");
                        exit(1);
                    }
                };
                Self::String(string)
            }
            Token::FalseValue => Self::False,
            Token::TrueValue => Self::True,
            Token::NullValue => Self::Null,
            Token::VoidValue => Self::Void,
            Token::BooleanValue => {
                let bool_value = match unit.value {
                    Some(Value::_Boolean(b)) => b,
                    _ => {
                        eprintln!("failed to unwrap a boolean");
                        exit(1);
                    }
                };
                if bool_value {
                    return Self::True;
                }
                Self::False
            }
            _ => Self::Null,
        }
    }
    pub fn is_truthy(&self) -> ValueType {
        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::String(s) => {
                if s.is_empty() {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::Map(x) => {
                if x.is_empty() {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::Boolean(b) => {
                if *b {
                    Self::True
                } else {
                    Self::False
                }
            }
            Self::False => Self::False,
            Self::True => Self::True,
            // TODO
            Self::Function(_) => Self::False,
            Self::DeclaredFunction(_) => Self::False,
            Self::Null => Self::False,
            Self::Void => Self::False,
        }
    }
}

impl Hash for Expression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}
impl Eq for Expression {}
impl Expression {
    pub fn get_id(&self) -> usize {
        match self {
            Expression::Binary { id, .. } => *id,
            Expression::Call { id, .. } => *id,
            Expression::Grouping { id, .. } => *id,
            Expression::Map { id, .. } => *id,
            Expression::Unary { id, .. } => *id,
            Expression::Value { id, .. } => *id,
            Expression::Variable { id, .. } => *id,
        }
    }
    pub fn evaluate(&self, module: Module) -> ValueType {
        match self {
            Expression::Map { id: _, items } => {
                let mut evaluated_map: Vec<ValueType> = Vec::new();

                for item in items {
                    evaluated_map.push(ValueType::String(item.0.to_string()));
                }
                ValueType::Map(evaluated_map)
            }
            Expression::Call {
                id: _,
                name,
                arguments,
            } => {
                let callable: ValueType = (*name).evaluate(module.clone());
                match callable {
                    ValueType::Function(function) => run_function(function, arguments, module),
                    ValueType::DeclaredFunction(fun) => {
                        let mut evaluated_args = vec![];
                        for arg in arguments {
                            evaluated_args.push(arg.evaluate(module.clone()));
                        }
                        fun.function.call(&evaluated_args)
                    }
                    _ => {
                        eprintln!("error: non-functional call");
                        ValueType::Null
                    }
                }
            }
            Expression::Variable { id: _, name } => match module.get(&name.lexeme, self.get_id()) {
                Some(value) => value.clone(),
                None => ValueType::Null,
            },
            Expression::Value { id: _, value } => (*value).clone(),
            Expression::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let lleft = left.evaluate(module.clone());
                let lright = right.evaluate(module.clone());
                let left_true = lleft.is_truthy();
                let _right_true = lright.is_truthy();
                match operator.token {
                    Token::Or => {
                        if left_true == ValueType::True {
                            return lleft;
                        }
                        return right.evaluate(module.clone());
                    }
                    Token::And => {
                        if left_true == ValueType::False {
                            return left_true;
                        }
                        return right.evaluate(module.clone());
                    }
                    _ => ValueType::Null,
                };
                match (&lleft, operator.token, &lright) {
                    (ValueType::Number(x), Token::Plus, ValueType::Number(y)) => {
                        ValueType::Number(x + y)
                    }
                    (ValueType::Number(x), Token::Minus, ValueType::Number(y)) => {
                        ValueType::Number(x - y)
                    }
                    (ValueType::Number(x), Token::Asteric, ValueType::Number(y)) => {
                        ValueType::Number(x * y)
                    }
                    (ValueType::Number(x), Token::Slash, ValueType::Number(y)) => {
                        ValueType::Number(x / y)
                    }
                    (ValueType::Number(x), Token::Percent, ValueType::Number(y)) => {
                        ValueType::Number(x % y)
                    }
                    (ValueType::Number(x), Token::More, ValueType::Number(y)) => {
                        ValueType::Boolean(x > y)
                    }
                    (ValueType::Number(x), Token::MoreEqual, ValueType::Number(y)) => {
                        ValueType::Boolean(x >= y)
                    }
                    (ValueType::Number(x), Token::Less, ValueType::Number(y)) => {
                        ValueType::Boolean(x < y)
                    }
                    (ValueType::Number(x), Token::LessEqual, ValueType::Number(y)) => {
                        ValueType::Boolean(x <= y)
                    }
                    (x, Token::EqualEqual, y) => ValueType::Boolean(x == y),
                    (x, Token::BangEqual, y) => ValueType::Boolean(x != y),
                    (_, _, _) => ValueType::Null,
                }
            }
            Expression::Grouping { id: _, expression } => expression.evaluate(module),
            Expression::Unary {
                id: _,
                left,
                operator,
            } => {
                let lleft = left.evaluate(module);
                match (lleft, operator.token) {
                    (ValueType::Number(x), Token::Minus) => ValueType::Number(-x),
                    (x, Token::Bang) => x.is_truthy(),
                    (_, _) => ValueType::Null,
                }
            }
        }
    }
}

pub fn run_function(
    function: FunctionValueType,
    arguments: &[Expression],
    module: Module,
) -> ValueType {
    if arguments.len() != function.parameter_count {
        eprintln!("invalid parameter count");
        exit(1)
    }
    let mut argument_values = vec![];
    for argument in arguments {
        argument_values.push(argument.evaluate(module.clone()))
    }
    let function_module = function.parent_module.enclose();
    for (i, value) in argument_values.iter().enumerate() {
        if i < function.parameters.len() {
            let (param_name_token, param_name_type) = &function.parameters[i];
            let param_name = &param_name_token.lexeme;
            let param_type = &param_name_type.lexeme;
            match (param_type.as_str(), value) {
                // TODO
                ("number", ValueType::Number(_)) => {}
                ("boolean", ValueType::Boolean(_)) => {}
                ("false", ValueType::False) => {}
                ("true", ValueType::True) => {}
                _ => {
                    eprintln!("error: invalid value type");
                    exit(1);
                }
            }
            function_module.define(param_name.clone(), value.clone());
        } else {
            eprintln!("number of called arguments doesnt match functions parameter count");
            exit(1);
        }
    }

    let mut int = Interpreter::new_with_module(function_module);
    for statement in function.body.iter() {
        int.interpret(vec![statement]);
        let value = int.specials.get("return");
        return if value.is_some() {
            if !(function.value_type.lexeme == value.clone().expect("TODO").to_string()) {
                eprintln!("invalid function output value type");
                exit(1);
            }
            value.clone().unwrap().clone()
        } else {
            ValueType::Void
        };
    }

    if function.value_type.lexeme != "void" {
        eprintln!("invalid function output value type");
        exit(1)
    }
    ValueType::Void
}
