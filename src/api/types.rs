use super::{statements::Statement, tokenlist::Unit};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Module {
    pub values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub public_values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub value_types: Rc<RefCell<HashMap<String, String>>>,
    pub public_value_types: Rc<RefCell<HashMap<String, String>>>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Module>>,
}

#[derive(Clone)]
pub struct FunctionValueType {
    pub parent_module: Module,
    pub name: String,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Box<Statement>>,
}

#[derive(Clone)]
pub struct AnonFunctionValueType {
    pub parent_module: Module,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Box<Statement>>,
}

#[derive(Clone)]
pub struct DeclaredFunctionValueType {
    pub name: String,
    pub parameter_count: usize,
    pub function: Rc<dyn Fn(&Vec<ValueType>) -> ValueType>,
}

#[derive(Clone)]
pub enum ValueType {
    NumberValueType(f32),
    StringValueType(String),
    MapValueType(Vec<ValueType>),
    FunctionValueType(DeclaredFunctionValueType),
    BooleanValueType(bool),
    TrueValueType,
    FalseValueType,
    NullValueType,
    VoidValueType,
}
