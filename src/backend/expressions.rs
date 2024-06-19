// TODO: backend handler of the expressions
use crate::api::{
    expressions::Expression,
    tokenlist::Unit,
    types::{Module, ValueType},
};

impl ValueType {
    pub fn from_unit(_unit: Unit) -> Self {
        Self::Null
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
            Expression::UnaryLeft { id, .. } => *id,
            Expression::UnaryRight { id, .. } => *id,
            Expression::Value { id, .. } => *id,
            Expression::Variable { id, .. } => *id,
            _ => 0,
        }
    }
    pub fn evaluate(&self, module: Module) -> ValueType {
        match self {
            // TODO
            _ => ValueType::Null,
        }
    }
}
