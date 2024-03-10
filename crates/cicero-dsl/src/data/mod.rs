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

use indexmap::IndexMap;
#[cfg(feature = "render")]
use minijinja::value::{StructObject, Value};
use serde::{Deserialize, Serialize};

use crate::types::EntityType;
#[cfg(feature = "render")]
use crate::types::TypeEnv;

#[cfg(feature = "render")]
pub mod ast;
#[cfg(feature = "render")]
use ast::Ast;

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    Struct(StructData),
    Enum(EnumData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StructData {
    type_name: String,
    fields: IndexMap<String, Data>,
    #[cfg(feature = "render")]
    methods: IndexMap<String, Ast>,
}

impl StructData {
    #[cfg(feature = "render")]
    pub fn ty<'a>(&self, type_env: &'a TypeEnv) -> Option<&'a EntityType> {
        type_env.get(&self.type_name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumData {
    type_name: String,
    // TODO: maybe use a usize?
    discriminant: String,
    fields: Vec<Data>,
}

impl EnumData {
    #[cfg(feature = "render")]
    pub fn ty<'a>(&self, type_env: &'a TypeEnv) -> Option<&'a EntityType> {
        type_env.get(&self.type_name)
    }
}

#[cfg(feature = "render")]
impl StructObject for StructData {
    fn get_field(&self, field: &str) -> Option<Value> {
        todo!()
    }
}
