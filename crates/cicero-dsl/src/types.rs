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

/// Fields of a struct or a variant of an enum.
pub type Fields = IndexMap<String, FieldType>;

pub type TypeEnv = HashMap<String, EntityType>;

/// A single step of a scenario.
///
/// The data, descripted by this structure, is only that is needed
/// to continue evaluation of the scenario.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScenarioStep {
    /// Name of the step.
    name: String,
    /// Header of the step, that may be displayed on top of data entry form.
    ///
    /// Usually it contains legal information, references to the law and
    /// warnings to the user.
    header: Option<MarkdownString>,
    /// Variables, that are needed to be filled in order to continue the
    /// scenario.
    variables: Vec<Variable>,
}

impl ScenarioStep {
    pub fn new(name: String, header: Option<MarkdownString>, variables: Vec<Variable>) -> Self {
        Self {
            name,
            header,
            variables,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn header(&self) -> Option<&str> {
        self.header.as_deref()
    }

    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }
}

/// A variable, that is needed to be filled in order to continue the scenario.
///
/// A variable is created by `let name: Ty` statement in the types of the
/// scenario.
#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
    /// Name of the variable.
    name: String,
    /// Comment, that should be displayed on top of the data entry field.
    comment: MarkdownString,
    /// Type of the variable.
    ty: Entity,
}

impl Variable {
    pub fn new(name: String, comment: MarkdownString, ty: Entity) -> Self {
        Self { name, comment, ty }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn ty(&self) -> &Entity {
        &self.ty
    }
}

/// A single enumeration type.
#[derive(Serialize, Deserialize, Debug)]
pub struct EnumType {
    name: String,
    variants: Vec<EnumVariantType>,
}

impl EnumType {
    pub fn new(name: String, variants: Vec<EnumVariantType>) -> Self {
        Self { name, variants }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn variants(&self) -> &[EnumVariantType] {
        &self.variants
    }

    /// Returns false if any of the variants has fields.
    pub fn is_simple(&self) -> bool {
        self.variants.iter().all(|variant| variant.is_simple())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumVariantType {
    name: String,
    comment: Option<MarkdownString>,
    fields: Vec<Entity>,
}

impl EnumVariantType {
    pub fn new(name: String, comment: Option<MarkdownString>, fields: Vec<Entity>) -> Self {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct StructType {
    name: String,
    comment: Option<MarkdownString>,
    fields: Fields,
    ancestor: Option<Box<StructType>>,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn ancestor(&self) -> Option<&StructType> {
        self.ancestor.as_deref()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldType {
    comment: MarkdownString,
    ty: Entity,
    required: bool,
}

impl FieldType {
    pub fn new(comment: MarkdownString, ty: Entity, required: bool) -> Self {
        Self {
            comment,
            ty,
            required,
        }
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn ty(&self) -> &Entity {
        &self.ty
    }

    pub fn required(&self) -> bool {
        self.required
    }
}

/// A single type of the scenario.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    name: String,
    ty: EntityType,
}

impl Entity {
    pub fn new(name: String, ty: EntityType) -> Self {
        Self { name, ty }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &EntityType {
        &self.ty
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
