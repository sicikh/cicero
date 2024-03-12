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

use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use super::ast;
use super::ast::TypeDef;
use crate::types;
use crate::types::VarEnv;

type TypeDefs = HashMap<String, TypeDef>;
type VarDefs = HashMap<String, ast::Variable>;

pub fn resolve(module: ast::Module) -> Result<types::Module, String> {
    let ast::Module {
        type_defs,
        variables,
    } = module;

    let type_defs = match find_type_decl_dups(type_defs) {
        Ok(defs) => defs,
        Err((_, dups)) => {
            return Err(format!("Duplicate type definitions: {:#?}", dups));
        },
    };

    let vars: VarEnv = match find_var_dups(variables) {
        Ok(var_defs) => {
            let mut resolved = HashMap::new();

            var_defs
                .into_values()
                .try_fold(HashMap::new(), |mut map: VarEnv, var| {
                    map.insert(var.name.clone(), types::Variable {
                        name: var.name,
                        comment: var.comment,
                        // TODO: handle error
                        ty: resolve_type(&var.ty, &type_defs, &mut HashSet::new(), &mut resolved)?,
                    });

                    Ok::<VarEnv, String>(map)
                })?
        },
        Err((_, dups)) => {
            return Err(format!("Duplicate type definitions: {:#?}", dups));
        },
    };

    Ok(types::Module { vars })
}

// TODO: typename in parser?
// TODO: errors: recursion
// TODO: methods
// TODO: move handling of predefined types to a resolved hashmap, think about it
// TODO: struct A: A {}
fn resolve_type(
    ty: &ast::Type,
    type_defs: &TypeDefs,
    visited: &mut HashSet<String>,
    resolved: &mut HashMap<String, types::EntityType>,
) -> Result<types::Entity, String> {
    let ast::Type { required, name } = ty;
    let required = *required;

    let type_name = match name.as_str() {
        "String" => Ok(types::EntityType::String),
        // TODO: more
        _ => Err("".to_string()),
    }
    .map(|ty| types::Entity { ty, required });

    if let Ok(ty) = type_name {
        return Ok(ty);
    }

    let def = match type_defs.get(name) {
        None => return Err(format!("Type definition not found: {}", name)),
        Some(def) => def,
    };

    if !visited.insert(def.name().to_string()) {
        return Err(format!("Recursion detected: {}", def.name()));
    }

    match def {
        TypeDef::Struct(s) => {
            let ast::Struct {
                comment,
                name,
                fields,
                parent,
                methods: _,
            } = s;

            let parent = {
                match parent
                    .as_ref()
                    .map(|p| {
                        // TODO: CHECK!!! is resolved.get() here is correct?
                        // can we create recursion here?

                        match resolved.get(p) {
                            Some(entity) => Ok(entity.clone()),
                            None => {
                                resolve_type(
                                    &ast::Type {
                                        required: false,
                                        name: p.clone(),
                                    },
                                    type_defs,
                                    // to prevent weird behavior:
                                    // `struct A: B { a: C } struct B { b: C }`
                                    // is not a recursion
                                    &mut visited.clone(),
                                    resolved,
                                )
                                .map(|entity| {
                                    resolved.insert(p.clone(), entity.ty.clone());
                                    entity.ty
                                })
                            },
                        }
                    })
                    .transpose()?
                {
                    Some(entity) => {
                        match entity {
                            types::EntityType::Struct(s) => Some(Box::new(s)),
                            _ => return Err("Parent type can't be an enum type".to_string()),
                        }
                    },
                    None => None,
                }
            };

            let fields = fields
                .iter()
                .map(|f| {
                    let ast::Field { comment, name, ty } = f;
                    let ty = resolve_type(ty, type_defs, visited, resolved)?;
                    Ok((name.clone(), types::Field::new(comment.clone(), ty)))
                })
                .collect::<Result<types::Fields, String>>()?;

            let struct_type = types::EntityType::Struct(types::Struct {
                name: name.clone(),
                comment: comment.clone(),
                fields,
                parent,
            });
            resolved.insert(name.clone(), struct_type.clone());
            let entity = types::Entity::new(struct_type, required);

            Ok(entity)
        },
        TypeDef::Enum(e) => {
            let ast::Enum {
                name,
                comment,
                variants,
                methods: _,
            } = e;

            let variants = variants
                .iter()
                .map(|v| {
                    let ast::EnumVariant {
                        comment,
                        name,
                        field,
                    } = v;

                    let field = field
                        .as_ref()
                        .map(|ty| resolve_type(ty, type_defs, visited, resolved))
                        .transpose()?;

                    let variant = types::EnumVariant {
                        name: name.clone(),
                        comment: comment.clone(),
                        field,
                    };

                    Ok(variant)
                })
                .collect::<Result<Vec<types::EnumVariant>, String>>()?;

            let enum_type = types::EntityType::Enum(types::Enum {
                name: name.clone(),
                comment: comment.clone(),
                variants,
            });
            resolved.insert(name.clone(), enum_type.clone());
            let entity = types::Entity::new(enum_type, required);

            Ok(entity)
        },
    }
}

// TODO: find overridings (e. g. `struct String { ... }`)
fn find_type_decl_dups(type_defs: Vec<TypeDef>) -> Result<TypeDefs, (TypeDefs, Vec<String>)> {
    type_defs
        .into_iter()
        .fold(Ok(HashMap::new()), |res, type_def| {
            let type_def_key = type_def.name().to_string();

            match res {
                Ok(mut defs) => {
                    match defs.insert(type_def_key.clone(), type_def) {
                        None => Ok(defs),
                        Some(_) => Err((defs, vec![type_def_key.clone()])),
                    }
                },
                Err((mut defs, mut dups)) => {
                    match defs.insert(type_def_key.clone(), type_def) {
                        None => Err((defs, dups)),
                        Some(_) => {
                            dups.push(type_def_key);
                            Err((defs, dups))
                        },
                    }
                },
            }
        })
}
// FIXME: dup code
fn find_var_dups(vars: Vec<ast::Variable>) -> Result<VarDefs, (VarDefs, Vec<String>)> {
    vars.into_iter().fold(Ok(HashMap::new()), |res, var| {
        let var_key = var.name.clone();

        match res {
            Ok(mut defs) => {
                match defs.insert(var_key.clone(), var) {
                    None => Ok(defs),
                    Some(_) => Err((defs, vec![var_key])),
                }
            },
            Err((mut defs, mut dups)) => {
                match defs.insert(var_key.clone(), var) {
                    None => Err((defs, dups)),
                    Some(_) => {
                        dups.push(var_key);
                        Err((defs, dups))
                    },
                }
            },
        }
    })
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use super::*;

    #[test]
    fn type_is_parent_and_field_test() {
        let ast_module = ast::Module {
            type_defs: vec![
                TypeDef::Struct(ast::Struct {
                    name: "A".to_string(),
                    comment: None,
                    fields: vec![],
                    parent: None,
                    methods: vec![],
                }),
                TypeDef::Struct(ast::Struct {
                    name: "B".to_string(),
                    comment: None,
                    fields: vec![ast::Field {
                        name: "a".to_string(),
                        comment: "Some comment".to_string(),
                        ty: ast::Type {
                            required: false,
                            name: "A".to_string(),
                        },
                    }],
                    parent: Some("A".to_string()),
                    methods: vec![],
                }),
            ],
            variables: vec![ast::Variable {
                name: "a".to_string(),
                comment: "Some comment".to_string(),
                ty: ast::Type {
                    required: false,
                    name: "B".to_string(),
                },
            }],
        };

        let module = resolve(ast_module).unwrap();
        let test = types::Module {
            vars: {
                let a_struct = types::Struct {
                    name: "A".to_string(),
                    comment: None,
                    fields: types::Fields::new(),
                    parent: None,
                };
                let a_entity =
                    types::Entity::new(types::EntityType::Struct(a_struct.clone()), false);

                let mut map = HashMap::new();
                map.insert("a".to_string(), types::Variable {
                    name: "a".to_string(),
                    comment: "Some comment".to_string(),
                    ty: types::Entity::new(
                        types::EntityType::Struct(types::Struct {
                            name: "B".to_string(),
                            comment: None,
                            fields: vec![(
                                "a".to_string(),
                                types::Field::new("Some comment".to_string(), a_entity),
                            )]
                            .into_iter()
                            .collect::<types::Fields>(),
                            parent: Some(Box::new(a_struct.clone())),
                        }),
                        false,
                    ),
                });
                map
            },
        };

        assert_eq!(module, test);
    }

    #[test]
    fn type_is_parent_of_itself() {
        let ast_module = ast::Module {
            type_defs: vec![TypeDef::Struct(ast::Struct {
                name: "A".to_string(),
                comment: None,
                fields: vec![],
                parent: Some("A".to_string()),
                methods: vec![],
            })],
            variables: vec![ast::Variable {
                name: "a".to_string(),
                comment: "Some comment".to_string(),
                ty: ast::Type {
                    required: false,
                    name: "A".to_string(),
                },
            }],
        };

        let module = resolve(ast_module);
        assert_eq!(module, Err("Recursion detected: A".to_string()));
    }
}
