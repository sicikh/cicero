use std::collections::HashMap;

use indexmap::IndexMap;

use super::*;
pub use crate::data::ast::Method;

pub type VarEnv = HashMap<String, Variable>;
pub type Methods = HashMap<String, Method>;
pub type TypeEnv = HashMap<String, Entity>;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub vars: VarEnv,
}
