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

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::types::EntityType;

#[cfg(feature = "render")]
pub mod expr;
#[cfg(feature = "render")]
use expr::Expr;

// TODO: move methods to here

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Var {
    pub name: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    Struct(Struct),
    Enum(Enum),
    String(String),
}

impl Display for Data {
    // TODO
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Struct(s) => write!(f, "{}", s.name),
            Data::Enum(e) => write!(f, "{}", e.name),
            Data::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: IndexMap<String, Data>,
    #[cfg(feature = "render")]
    #[serde(skip)]
    pub methods: Arc<HashMap<String, Expr>>,
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub name: String,
    // TODO: maybe use a usize?
    pub discriminant: String,
    pub field: Option<Box<Data>>,
    #[cfg(feature = "render")]
    #[serde(skip)]
    pub methods: Arc<HashMap<String, Expr>>,
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
