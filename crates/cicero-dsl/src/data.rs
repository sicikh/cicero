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

use serde::{Deserialize, Serialize};

#[cfg(feature = "render")]
use crate::types::{self, EntityType};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Var {
    pub name: String,
    pub data: Data,
}

// NB: Data does not contain a None variant,
//  because minijinja does not have instrument to handle it...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Data {
    Struct(Struct),
    Enum(Enum),
    String(String),
    Array(Array),
}

impl Data {
    #[cfg(feature = "render")]
    pub fn is_type(&self, ty: &EntityType) -> bool {
        match (self, ty) {
            (Data::Struct(data_struct), EntityType::Struct(type_struct)) => {
                data_struct.is_type(type_struct)
            },
            (Data::Enum(data_enum), EntityType::Enum(type_enum)) => data_enum.is_type(type_enum),
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
            Data::Array(a) => {
                write!(
                    f,
                    "[{}]",
                    a.inner
                        .iter()
                        .map(|d| d.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: HashMap<String, Data>,
}

impl Struct {
    #[cfg(feature = "render")]
    pub fn is_type(&self, type_struct: &types::Struct) -> bool {
        self.name == type_struct.name
            // all data fields are present in the type
            && self.fields.iter().all(|(field_name, data_field)| {
            type_struct.fields
                .get(field_name)
                .map_or(false, |type_field| data_field.is_type(&type_field.entity.ty))
        })
            // all required type fields are present in the data
            && type_struct.fields.iter().filter(|&(_, type_field)| type_field.entity.is_required).all(|(field_name, type_field)| {
            self.fields
                .get(field_name)
                .map_or(false, |data_field| data_field.is_type(&type_field.entity.ty))
        })
    }
}

impl Display for Struct {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub discriminant: String,
    pub field: Option<Box<Data>>,
}

impl Enum {
    #[cfg(feature = "render")]
    pub fn is_type(&self, type_enum: &types::Enum) -> bool {
        self.name == type_enum.name
            && match type_enum.variants.get(&self.discriminant) {
                Some(type_variant) => {
                    match (&type_variant.field, &self.field) {
                        (Some(variant_type_field), Some(data_field)) => {
                            data_field.is_type(&variant_type_field.ty)
                        },
                        (Some(variant_type_field), None) => !variant_type_field.is_required,
                        (None, None) => true,
                        (None, Some(_)) => false,
                    }
                },
                None => false,
            }
    }
}

impl Display for Enum {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Array {
    pub inner: Vec<Data>,
}
