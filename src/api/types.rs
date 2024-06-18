use std::{cell::RefCell, collections::HashMap, rc::Rc};
use super::{statements::Statement, tokenlist::Unit};

#[derive(Clone, Debug)]
pub struct Module {
    pub values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub exported_values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub value_types: Rc<RefCell<HashMap<String, String>>>,
    pub exported_value_types: Rc<RefCell<HashMap<String, String>>>,
    locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Module>>,
}

#[derive(Debug, Clone)]
pub struct FunctionValueType {
    pub parent_module: Module,
    pub name: String,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Box<Statement>>,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    NumberValueType(f32),
    StringValueType(String),
    MapValueType(Vec<ValueType>),
    FunctionValueType(FunctionValueType),
    BooleanValueType(bool),
    TrueValueType,
    FalseValueType,
    NullValueType,
    VoidValueType,
}
