// TODO: backend handler of the expressions
use crate::api::{
    expressions::Expression,
    tokenlist::Unit,
    types::{Module, ValueType},
};

impl ValueType {
    // TODO
    pub fn from_unit(_unit: Unit) -> Self {
        Self::Null
    }
    pub fn is_truthy(&self) -> ValueType {
        match self {
            Self::Number(x) => {
                if *x == 0.0 as f32 {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::String(s) => {
                if s.len() == 0 {
                    Self::False
                } else {
                    Self::True
                }
            }
            Self::Map(x) => {
                if x.len() == 0 {
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
            Self::Null => Self::False,
            Self::Void => Self::False,
        }
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
    pub fn evaluate(&self, _module: Module) -> ValueType {
        match self {
            // TODO
            _ => ValueType::Null,
        }
    }
}
