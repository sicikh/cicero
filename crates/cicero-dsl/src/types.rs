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

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub type MarkdownString = String;

#[cfg(feature = "render")]
use crate::data::ast::Method;

/// Fields of a struct or a variant of an enum.
pub type Fields = IndexMap<String, FieldType>;

#[cfg(feature = "render")]
pub type TypeEnv = HashMap<String, Entity>;
#[cfg(feature = "render")]
pub type VarEnv = HashMap<String, Variable>;
#[cfg(feature = "render")]
pub type Methods = HashMap<String, Method>;

/// Metadata of the single scenario.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioMeta {
    pub id: u64,
    pub name: String,
    pub description: MarkdownString,
    pub date_of_creation: String,
    pub date_of_last_change: String,
    pub author: String,
}

impl ScenarioMeta {
    pub fn new(
        id: u64,
        name: String,
        description: MarkdownString,
        date_of_creation: String,
        date_of_last_change: String,
        author: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            date_of_creation,
            date_of_last_change,
            author,
        }
    }
}

impl PartialEq for ScenarioMeta {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for ScenarioMeta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

/// A single step of a scenario.
///
/// The data, descripted by this structure, is only that is needed
/// to continue evaluation of the scenario.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScenarioStep {
    /// Name of the step.
    pub name: String,
    /// Header of the step, that may be displayed on top of data entry form.
    ///
    /// Usually it contains legal information, references to the law and
    /// warnings to the user.
    pub header: Option<MarkdownString>,
    /// Variables, that are needed to be filled in order to continue the
    /// scenario.
    pub variables: Vec<Variable>,
    /// Is step of the first phase of the scenario, when the render is not
    /// ready.
    pub is_first_phase: bool,
}

impl PartialEq for ScenarioStep {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl ScenarioStep {
    pub fn new(
        name: String,
        header: Option<MarkdownString>,
        variables: Vec<Variable>,
        is_first_phase: bool,
    ) -> Self {
        Self {
            name,
            header,
            variables,
            is_first_phase,
        }
    }
}

/// A variable, that is needed to be filled in order to continue the scenario.
///
/// A variable is created by `let name: Ty` statement in the types of the
/// scenario.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variable {
    /// Name of the variable.
    pub name: String,
    /// Comment, that should be displayed on top of the data entry field.
    pub comment: MarkdownString,
    /// Type of the variable.
    pub ty: Entity,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Variable {
    pub fn new(name: String, comment: MarkdownString, ty: Entity) -> Self {
        Self { name, comment, ty }
    }
}

/// A single enumeration type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub variants: Vec<EnumVariantType>,
}

impl PartialEq for EnumType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl EnumType {
    pub fn new(
        name: String,
        comment: Option<MarkdownString>,
        variants: Vec<EnumVariantType>,
    ) -> Self {
        Self {
            name,
            comment,
            variants,
        }
    }

    /// Returns false if any of the variants has fields.
    pub fn is_simple(&self) -> bool {
        self.variants.iter().all(|variant| variant.is_simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumVariantType {
    pub name: String,
    pub comment: MarkdownString,
    pub fields: Vec<Entity>,
}

impl PartialEq for EnumVariantType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl EnumVariantType {
    pub fn new(name: String, comment: MarkdownString, fields: Vec<Entity>) -> Self {
        Self {
            name,
            comment,
            fields,
        }
    }

    /// Returns false if the variant has fields.
    pub fn is_simple(&self) -> bool {
        self.fields.is_empty()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub fields: Fields,
    pub ancestor: Option<Box<StructType>>,
}

impl PartialEq for StructType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl StructType {
    pub fn new(
        name: String,
        comment: Option<MarkdownString>,
        fields: Fields,
        ancestor: Option<Box<StructType>>,
    ) -> Self {
        Self {
            name,
            comment,
            fields,
            ancestor,
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&FieldType> {
        self.ancestor
            .as_deref()
            .and_then(|ancestor| ancestor.get_field(name))
            // FIXME: eagerly evaluated at this moment, profile and check
            .or(self.fields.get(name))
    }

    pub fn is_descendant(&self) -> bool {
        self.ancestor.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldType {
    pub comment: MarkdownString,
    pub ty: Entity,
}

impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

impl FieldType {
    pub fn new(comment: MarkdownString, ty: Entity) -> Self {
        Self { comment, ty }
    }
}

/// A single type of the scenario.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entity {
    pub ty: EntityType,
    pub required: bool,
}

impl Entity {
    pub fn new(ty: EntityType, required: bool) -> Self {
        Self { ty, required }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EntityType {
    String,
    Number,
    PhoneNumber,
    Date,
    Place,
    Enum(EnumType),
    Struct(StructType),
}

// TODO: think about this
// impl EntityType {
//     pub fn validate_regex(&self) ->
// }
