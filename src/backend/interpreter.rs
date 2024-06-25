use std::{collections::HashMap, process::exit, rc::Rc};

use crate::api::{
    tokenlist::Unit,
    types::{
        ClosuresWrapper, DeclaredFunctionValueType, FunctionValueType, Module, ValueType,
        ValueTypeFunction,
    },
    Statement,
};

pub struct Interpreter {
    pub specials: HashMap<String, ValueType>,
    pub module: Module,
}

pub fn declare_builtin(
    name: String,
    arguments: usize,
    function: Rc<dyn ValueTypeFunction>,
    module: &mut Module,
) {
    module.define(
        name.clone(),
        ValueType::DeclaredFunction(DeclaredFunctionValueType {
            name: name.clone(),
            parameter_count: arguments,
            function,
        }),
    );
}
impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            specials: HashMap::new(),
            module: Module::new(HashMap::new()),
        };
        // clippy::init_numbered_fields
        declare_builtin(
            "print!".to_string(),
            1,
            Rc::new(ClosuresWrapper {
                0: Box::new(|args: &[ValueType]| {
                    if args.len() == 1 {
                        println!("{:?}", args[0]);
                    } else {
                        eprint!("invalid number of arguments for print!");
                        exit(1);
                    }
                    ValueType::Null
                }),
            }),
            &mut interpreter.module,
        );

        interpreter
    }
    pub fn new_with_module(module: Module) -> Self {
        Self {
            specials: HashMap::new(),
            module,
        }
    }
    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        self.module.resolve(locals);
    }
    pub fn interpret(&mut self, statements: Vec<&Statement>) {
        for statement in statements {
            match statement {
                Statement::Expression { expression } => {
                    expression.evaluate(self.module.clone());
                }
                Statement::Block { statements } => {
                    let new_module = self.module.enclose();
                    let old_module = self.module.clone();
                    self.module = new_module;
                    self.interpret((*statements).iter().collect());
                    self.module = old_module;
                }
                Statement::Function {
                    name,
                    parameters: _,
                    value_type: _,
                    body: _,
                    is_public,
                } => {
                    let callable = self.wrap_function(statement);
                    let function = ValueType::Function(callable);
                    if *is_public {
                        self.module
                            .define_public(name.lexeme.clone(), function.clone());
                        self.module.define(name.lexeme.clone(), function);
                    } else {
                        self.module.define(name.lexeme.clone(), function)
                    }
                }
                Statement::If {
                    condition,
                    body,
                    else_if_branches,
                    else_branch,
                } => {
                    let truth_value = condition.evaluate(self.module.clone());
                    let is_true = truth_value.is_truthy() != ValueType::True;

                    if is_true {
                        self.interpret(vec![body]);
                    } else {
                        let executed = false;
                        for (else_if_predicates, else_if_statement) in else_if_branches {
                            let mut all_true = true;
                            for else_if_predicate in else_if_predicates {
                                let else_if_truth_value =
                                    else_if_predicate.evaluate(self.module.clone());
                                if else_if_truth_value.is_truthy() != ValueType::True {
                                    all_true = false;
                                    break;
                                }
                            }
                            if all_true {
                                self.interpret(vec![else_if_statement]);
                                break;
                            }
                        }
                        if !executed {
                            if let Some(else_statement) = else_branch {
                                self.interpret(vec![else_statement])
                            }
                        }
                    }
                }
                Statement::Module { name: _, from: _ } => {
                    // TODO
                }
                Statement::Return { value } => {
                    let eval = value.evaluate(self.module.clone());
                    self.specials.insert("return".to_string(), eval);
                }
            }
        }
    }
    pub fn wrap_function(&self, statement: &Statement) -> FunctionValueType {
        if let Statement::Function {
            name,
            parameters,
            value_type,
            body,
            is_public,
        } = statement
        {
            let parameters: Vec<(Unit, Unit)> = parameters
                .iter()
                .map(|(name, unit)| (name.clone(), unit.clone()))
                .collect();
            let body: Vec<Statement> = body.iter().map(|b| (*b).clone()).collect();
            FunctionValueType {
                name: name.lexeme.clone(),
                parameter_count: parameters.len(),
                parameters,
                parent_module: self.module.clone(),
                body,
                value_type: value_type.clone(),
                is_public: *is_public,
            }
        } else {
            eprintln!("failed to declare a function");
            exit(1);
        }
    }
}
