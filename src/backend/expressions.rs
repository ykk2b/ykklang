// TODO: backend handler of the expressions
use crate::api::{tokenlist::Unit, types::ValueType};

impl ValueType {
    pub fn from_unit(_unit: Unit) -> Self {
        Self::NullValueType
    }
}
