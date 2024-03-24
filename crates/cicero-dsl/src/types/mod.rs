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
use std::hash::Hash;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub type MarkdownString = String;

/// Fields of a struct.
pub type Fields = IndexMap<String, Field>;

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
/// The data, described by this structure, is only that is needed
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
    /// Variables, that are needed to be filled to continue the
    /// scenario.
    pub variables: Vec<Var>,
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
        variables: Vec<Var>,
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

/// A variable, that is needed to be filled to continue the scenario.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Var {
    /// Name of the variable.
    pub name: String,
    /// Comment, that should be displayed on top of the data entry field.
    pub comment: MarkdownString,
    /// Type of the variable.
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
    pub fn new(name: String, comment: MarkdownString, ty: Entity) -> Self {
        Self { name, comment, ty }
    }
}

/// A single enumeration type.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub variants: IndexMap<String, EnumVariant>,
}

impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Enum {
    pub fn new(
        name: String,
        comment: Option<MarkdownString>,
        variants: IndexMap<String, EnumVariant>,
    ) -> Self {
        Self {
            name,
            comment,
            variants,
        }
    }

    /// Returns false if any of the variants has fields.
    pub fn is_simple(&self) -> bool {
        self.variants.values().all(|variant| variant.is_simple())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub comment: MarkdownString,
    pub field: Option<Entity>,
}

impl PartialEq for EnumVariant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl EnumVariant {
    pub fn new(name: String, comment: MarkdownString, field: Option<Entity>) -> Self {
        Self {
            name,
            comment,
            field,
        }
    }

    /// Returns false if the variant has field.
    pub fn is_simple(&self) -> bool {
        self.field.is_none()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub comment: Option<MarkdownString>,
    pub fields: Fields,
    pub parent: Option<Box<Struct>>,
}

impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Struct {
    pub fn new(
        name: String,
        comment: Option<MarkdownString>,
        fields: Fields,
        parent: Option<Box<Struct>>,
    ) -> Self {
        Self {
            name,
            comment,
            fields,
            parent,
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.parent
            .as_deref()
            .and_then(|ancestor| ancestor.get_field(name))
            .or_else(|| self.fields.get(name))
    }

    pub fn is_descendant(&self) -> bool {
        self.parent.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub comment: MarkdownString,
    pub entity: Entity,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl Field {
    pub fn new(comment: MarkdownString, entity: Entity) -> Self {
        Self { comment, entity }
    }
}

/// A single type of the scenario.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entity {
    pub ty: EntityType,
    pub is_required: bool,
}

impl Entity {
    pub fn new(ty: EntityType, is_required: bool) -> Self {
        Self { ty, is_required }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EntityType {
    String,
    Integer,
    PhoneNumber,
    Date,
    Place,
    Enum(Enum),
    Struct(Struct),
}
