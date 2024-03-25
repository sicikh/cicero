/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>,
 * Gleb Krylov <gleb_cry@mail.ru>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use crate::types::MarkdownString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub type_defs: Vec<TypeDef>,
    pub variables: Vec<Variable>,
}

impl Module {
    pub fn extend(&mut self, other: Module) {
        self.type_defs.extend(other.type_defs);
        self.variables.extend(other.variables);
    }

    pub fn extend_type_def(&mut self, type_defs: Vec<TypeDef>) {
        self.type_defs.extend(type_defs);
    }

    pub fn extend_variables(&mut self, variables: Vec<Variable>) {
        self.variables.extend(variables);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDef {
    Struct(Struct),
    Enum(Enum),
}

impl TypeDef {
    pub fn name(&self) -> &str {
        match self {
            TypeDef::Struct(s) => &s.name,
            TypeDef::Enum(e) => &e.name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub fields: Vec<Field>,
    pub parent: Option<String>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method {
    pub name: String,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub comment: MarkdownString,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub is_array: bool,
    pub is_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub variants: Vec<EnumVariant>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: String,
    pub comment: MarkdownString,
    pub field: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub comment: MarkdownString,
    pub ty: Type,
}
// TODO
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Todo,
}
