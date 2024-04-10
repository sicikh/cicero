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

use cicero_dsl::{data as dsl, types};
use indexmap::IndexMap;
use leptos::*;

#[derive(Clone)]
pub enum Data {
    Struct(RwSignal<Struct>),
    String(RwSignal<String>),
    Enum(RwSignal<Enum>),
    Array(RwSignal<Array>),
}

#[derive(Clone)]
pub struct Struct {
    pub name: String,
    pub fields: IndexMap<String, RwSignal<Data>>,
}

impl From<dsl::Struct> for Struct {
    fn from(s: dsl::Struct) -> Self {
        Struct {
            name: s.name,
            fields: s
                .fields
                .into_iter()
                .map(|(k, v)| (k, RwSignal::new(v.into())))
                .collect(),
        }
    }
}

impl From<Struct> for dsl::Struct {
    fn from(s: Struct) -> Self {
        dsl::Struct {
            name: s.name,
            fields: s
                .fields
                .into_iter()
                .map(|(name, field)| (name, field().into()))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Enum {
    pub name: String,
    pub discriminant: String,
    pub field: Option<RwSignal<Data>>,
}

impl From<dsl::Enum> for Enum {
    fn from(e: dsl::Enum) -> Self {
        Enum {
            name: e.name,
            discriminant: e.discriminant,
            field: e.field.map(|f| RwSignal::new((*f).into())),
        }
    }
}

impl From<Enum> for dsl::Enum {
    fn from(e: Enum) -> Self {
        dsl::Enum {
            name: e.name,
            discriminant: e.discriminant,
            field: e.field.map(|f| Box::new(f().into())),
        }
    }
}

#[derive(Clone)]
pub struct Array {
    pub inner: Vec<RwSignal<Data>>,
}

impl From<dsl::Array> for Array {
    fn from(a: dsl::Array) -> Self {
        Array {
            inner: a
                .inner
                .into_iter()
                .map(|data| RwSignal::new(data.into()))
                .collect(),
        }
    }
}

impl From<Array> for dsl::Array {
    fn from(a: Array) -> Self {
        dsl::Array {
            inner: a.inner.into_iter().map(|data| data().into()).collect(),
        }
    }
}

impl From<dsl::Data> for Data {
    fn from(data: dsl::Data) -> Self {
        match data {
            dsl::Data::Struct(s) => Data::Struct(RwSignal::new(Struct::from(s))),
            dsl::Data::String(s) => Data::String(RwSignal::new(s)),
            dsl::Data::Enum(e) => Data::Enum(RwSignal::new(Enum::from(e))),
            dsl::Data::Array(a) => Data::Array(RwSignal::new(Array::from(a))),
        }
    }
}

impl From<Data> for dsl::Data {
    fn from(data: Data) -> Self {
        match data {
            Data::Struct(s) => dsl::Data::Struct(s().into()),
            Data::String(s) => dsl::Data::String(s()),
            Data::Enum(e) => dsl::Data::Enum(e().into()),
            Data::Array(a) => dsl::Data::Array(a().into()),
        }
    }
}

pub fn data_from_entity(entity_type: &types::EntityType) -> Data {
    match entity_type {
        types::EntityType::String => Data::String(RwSignal::new(String::new())),
        types::EntityType::Struct(structure) => {
            Data::Struct(RwSignal::new(Struct {
                name: structure.name.clone(),
                fields: structure
                    .fields
                    .iter()
                    .map(|(name, field)| {
                        (
                            name.clone(),
                            RwSignal::new(data_from_entity(&field.entity.ty)),
                        )
                    })
                    .collect(),
            }))
        },
        types::EntityType::Enum(enumeration) => {
            let (variant_name, variant) = enumeration.variants.iter().next().unwrap();
            Data::Enum(RwSignal::new(Enum {
                name: enumeration.name.clone(),
                discriminant: variant_name.clone(),
                field: variant
                    .field
                    .as_ref()
                    .map(|entity| RwSignal::new(data_from_entity(&entity.ty))),
            }))
        },
        types::EntityType::Array(array) => Data::Array(RwSignal::new(Array { inner: vec![] })),
        types::EntityType::Integer => todo!(),
        types::EntityType::PhoneNumber => todo!(),
        types::EntityType::Date => todo!(),
        types::EntityType::Place => todo!(),
    }
}
