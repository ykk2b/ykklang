use std::{
    hash::{Hash, Hasher},
    process::exit,
};

use crate::api::{
    expressions::Expression,
    statements::Statement,
    tokenlist::{Token, Unit, Value},
    types::{AnonFunctionValueType, FunctionValueType, Module, ValueType},
};

use super::interpreter::Interpreter;

impl ValueType {
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
            Self::AnonFunction(_) => Self::False,
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
impl Expression {
    pub fn get_id(&self) -> usize {
        match self {
            Expression::Anonymous { id, .. } => *id,
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
            Expression::Anonymous {
                id: _,
                parameters,
                value_type,
                body,
            } => {
                let function = AnonFunctionValueType {
                    parameters: parameters.clone(),
                    parameter_count: parameters.len().clone(),
                    parent_module: module.clone(),
                    value_type: value_type.clone(),
                    body: body.clone(),
                };
                let callback = module.enclose();

                let int = Interpreter::new_with_module(callback.clone());
                let callable = int.wrap_function(&Statement::Function {
                    name: Unit {
                        token: Token::Identifier,
                        lexeme: "anon".to_string(),
                        value: None,
                        line_number: value_type.clone().line_number,
                    },
                    parameters: function.clone().parameters,
                    value_type: function.clone().value_type,
                    body: function.clone().body,
                    is_public: false,
                });
                let fun = ValueType::Function(callable.clone());
                module.define("anon".to_string(), fun);
                ValueType::AnonFunction(function)
            }
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
                    // TODO
                    (_, _, _) => ValueType::Null,
                }
            }
            Expression::Grouping { id: _, expression } => expression.evaluate(module),
            Expression::Unary {
                id: _,
                left,
                operator,
            } => {
                let _lleft = left.evaluate(module);
                match (&left, operator.token) {
                    // TODO
                    (_, _) => ValueType::Null,
                }
            }
        }
    }
}

pub fn run_function(
    _function: FunctionValueType,
    _arguments: &Vec<Expression>,
    _module: Module,
) -> ValueType {
    // TODO
    ValueType::Void
}
