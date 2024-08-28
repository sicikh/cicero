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

use std::hash::Hash;

use serde::{Deserialize, Serialize};

pub type HtmlString = String;

/// Fields of a struct.
pub type Fields = Vec<Field>;

/// A variable, that is needed to be filled to continue the scenario.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Var {
    /// Name of the variable.
    pub name: String,
    /// Comment, that should be displayed on top of the data entry field.
    pub comment: HtmlString,
    /// Type of the variable.
    #[serde(flatten)]
    pub ty: Entity,
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Var {}

impl Hash for Var {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Var {
    pub fn new(name: String, comment: HtmlString, ty: Entity) -> Self {
        Self { name, comment, ty }
    }
}

/// A single enumeration type.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    #[serde(rename = "typeName")]
    pub name: String,
    #[serde(rename = "typeNameComment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<HtmlString>,
    pub variants: Vec<EnumVariant>,
}

impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Enum {
    /// Returns false if any of the variants has fields.
    pub fn is_simple(&self) -> bool {
        self.variants.iter().all(|variant| variant.is_simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumVariant {
    pub name: String,
    pub comment: HtmlString,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub field: Option<Entity>,
}

impl PartialEq for EnumVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl EnumVariant {
    /// Returns false if the variant has field.
    pub fn is_simple(&self) -> bool {
        self.field.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Struct {
    #[serde(rename = "typeName")]
    pub name: String,
    #[serde(rename = "typeNameComment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<HtmlString>,
    pub fields: Fields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Struct>>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Struct {
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.parent
            .as_deref()
            .and_then(|ancestor| ancestor.get_field(name))
            .or_else(|| self.fields.iter().find(|&field| field.name == name))
    }

    pub fn is_descendant(&self) -> bool {
        self.parent.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub comment: HtmlString,
    #[serde(flatten)]
    pub entity: Entity,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Array {
    #[serde(rename = "elementType")]
    pub ty: Box<EntityType>,
}

/// A single type of the scenario.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[serde(flatten)]
    pub ty: EntityType,
    pub is_required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum EntityType {
    String,
    Integer,
    PhoneNumber,
    Date,
    Place,
    Enum(Enum),
    Struct(Struct),
    Array(Array),
}
