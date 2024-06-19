// TODO: backend handler of the expressions
use crate::api::{
    expressions::Expression,
    tokenlist::Unit,
    types::{Module, ValueType},
};

impl ValueType {
    pub fn from_unit(_unit: Unit) -> Self {
        Self::NullValueType
    }
}
impl Expression {
    pub fn get_id(&self) -> usize {
        match self {
            Expression::AnonymousFunctionExpression { id, .. } => *id,
            Expression::BinaryExpression { id, .. } => *id,
            Expression::FunctionCallExpression { id, .. } => *id,
            Expression::GroupingExpression { id, .. } => *id,
            Expression::MapExpression { id, .. } => *id,
            Expression::UnaryLeftExpression { id, .. } => *id,
            Expression::UnaryRightExpression { id, .. } => *id,
            Expression::ValueExpression { id, .. } => *id,
            Expression::VariableExpression { id, .. } => *id,
            _ => 0,
        }
    }
    pub fn evaluate(&self, module: Module) -> ValueType {
        match self {
            // TODO
            _ => ValueType::NullValueType,
        }
    }
}
