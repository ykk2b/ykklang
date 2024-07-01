use std::{cell::RefCell, collections::HashMap, process::exit, rc::Rc};

use crate::{api::types::{Module, ValueType}, log};


fn get_values() -> Rc<RefCell<HashMap<String, ValueType>>> {
    let env = HashMap::new();
    Rc::new(RefCell::new(env))
}

impl Module {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        Self {
            values: get_values(),
            public_values: get_values(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        }
    }
    pub fn enclose(&self) -> Module {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            public_values: Rc::new(RefCell::new(HashMap::new())),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        }
    }
    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        for (key, value) in locals.iter() {
            self.locals.borrow_mut().insert(*key, *value);
        }
    }
    pub fn define(&self, name: String, value: ValueType) {
        self.values.borrow_mut().insert(name, value);
    }
    pub fn define_public(&self, name: String, value: ValueType) {
        self.public_values.borrow_mut().insert(name, value);
    }
    pub fn get(&self, name: &str, expression_id: usize) -> Option<ValueType> {
        let distance = self.locals.borrow().get(&expression_id).cloned();
        self.get_internal(name, distance)
    }
    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<ValueType> {
        if distance.is_none() {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get_internal(name, distance),
            }
        } else {
            let distance = distance.expect("failed to get distance");
            if distance == 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    None => {
                        log("failed to resolve a variable");
                        exit(1);
                    }
                    Some(env) => {
                        assert!(distance > 0);
                        env.get_internal(name, Some(distance - 1))
                    }
                }
            }
        }
    }
}
