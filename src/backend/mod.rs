use crate::api::statements::Statement;

pub mod expressions;
pub mod modules;
pub mod resolver;

pub fn backend(_statements: Result<Vec<Statement>, String>) {}
