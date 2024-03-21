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

use crate::types::{self, Entity, EntityType};

#[cfg(feature = "render")]
pub mod expr;
#[cfg(feature = "render")]
pub use expr::Expr;
#[cfg(feature = "render")]
pub type Methods = HashMap<String, Expr>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Var {
    pub name: String,
    pub data: Data,
}

// NB: Data does not contain a None variant,
// because minijinja does not have instrument to handle it...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    Struct(Struct),
    Enum(Enum),
    String(String),
}

impl Data {
    pub fn is_ty(&self, ty: &EntityType) -> bool {
        match (self, ty) {
            (Data::Struct(data_struct), EntityType::Struct(ty_struct)) => {
                data_struct.is_ty(ty_struct)
            },
            (Data::Enum(data_enum), EntityType::Enum(ty_enum)) => data_enum.is_ty(ty_enum),
            (Data::String(_), EntityType::String) => true,
            _ => false,
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Struct(s) => write!(f, "`struct {}`", s.name),
            Data::Enum(e) => write!(f, "`enum {}`", e.name),
            Data::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, Data>,
    #[cfg(feature = "render")]
    #[serde(skip)]
    pub methods: Option<Arc<Methods>>,
}

// TODO: move to render feature
// TODO: bool -> Result<(), String>
impl Struct {
    pub fn is_ty(&self, ty: &types::Struct) -> bool {
        self.name == ty.name
            // all data fields are present in the type
            && self.fields.iter().all(|(name, field)| {
                ty.fields
                    .get(name)
                    .map_or(false, |ty_field| field.is_ty(&ty_field.ty.ty))
            })
            // all required type fields are present in the data
            && ty.fields.iter().filter(|(_, field)| field.ty.is_required).all(|(name, field_ty)| {
                self.fields
                    .get(name)
                    // TODO: find more adequate naming... tytyty lol
                    .map_or(false, |field| field.is_ty(&field_ty.ty.ty))
            })
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub discriminant: String,
    pub field: Option<Box<Data>>,
    #[cfg(feature = "render")]
    #[serde(skip)]
    pub methods: Option<Arc<Methods>>,
}

impl Enum {
    pub fn is_ty(&self, ty: &types::Enum) -> bool {
        self.name == ty.name
            && match ty
                .variants
                .iter()
                .find(|variant| variant.name == self.discriminant)
            {
                Some(variant) => {
                    match (&variant.field, &self.field) {
                        (Some(variant_field), Some(self_field)) => {
                            self_field.is_ty(&variant_field.ty)
                        },
                        (Some(variant_field), None) => !variant_field.is_required,
                        (None, None) => true,
                        (None, Some(_)) => false,
                    }
                },
                None => false,
            }
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
