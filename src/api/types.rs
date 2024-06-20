use super::{statements::Statement, tokenlist::Unit};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq, Debug)]
pub struct Module {
    pub values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub public_values: Rc<RefCell<HashMap<String, ValueType>>>,
    pub value_types: Rc<RefCell<HashMap<String, String>>>,
    pub public_value_types: Rc<RefCell<HashMap<String, String>>>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Module>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionValueType {
    pub parent_module: Module,
    pub name: String,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Statement>,
    pub is_public: bool,
}

trait ValueTypeFunction {
    fn call(&self, args: &Vec<ValueType>) -> ValueType;
}

struct ClosuresWrapper(Box<dyn Fn(&Vec<ValueType>) -> ValueType>);

impl ValueTypeFunction for ClosuresWrapper {
    fn call(&self, args: &Vec<ValueType>) -> ValueType {
        (self.0)(args)
    }
}

#[derive(Clone)]
pub struct AnonFunctionValueType {
    pub parent_module: Module,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Statement>,
}

#[derive(Clone)]
pub struct DeclaredFunctionValueType {
    pub name: String,
    pub parameter_count: usize,
    pub function: Rc<dyn ValueTypeFunction>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ValueType {
    Number(f32),
    String(String),
    Map(Vec<ValueType>),
    Function(FunctionValueType),
    Boolean(bool),
    True,
    False,
    Null,
    Void,
}
