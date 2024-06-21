use super::{statements::Statement, tokenlist::Unit};
use core::fmt;
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

pub trait ValueTypeFunction {
    fn call(&self, args: &Vec<ValueType>) -> ValueType;
}

pub trait RcValueTypeFunctionEq {
    fn rc_eq(&self, other: &Self) -> bool;
}

impl RcValueTypeFunctionEq for Rc<dyn ValueTypeFunction> {
    fn rc_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

pub struct ClosuresWrapper(pub Box<dyn Fn(&Vec<ValueType>) -> ValueType>);

impl ValueTypeFunction for ClosuresWrapper {
    fn call(&self, args: &Vec<ValueType>) -> ValueType {
        (self.0)(args)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AnonFunctionValueType {
    pub parent_module: Module,
    pub parameter_count: usize,
    pub parameters: Vec<(Unit, Unit)>,
    pub value_type: Unit,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct DeclaredFunctionValueType {
    pub name: String,
    pub parameter_count: usize,
    pub function: Rc<dyn ValueTypeFunction>,
}

impl PartialEq for DeclaredFunctionValueType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.parameter_count == other.parameter_count
            && self.function.rc_eq(&other.function)
    }
}

impl fmt::Debug for dyn ValueTypeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValueTypeFunction")
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ValueType {
    Number(f32),
    String(String),
    Map(Vec<ValueType>),
    Function(FunctionValueType),
    AnonFunction(AnonFunctionValueType),
    _DeclaredFunction(DeclaredFunctionValueType),
    Boolean(bool),
    True,
    False,
    Null,
    Void,
}