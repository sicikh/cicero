/*
 * Copyright (C) 2024 Kirill Lukashev <kirill.lukashev.sic@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use super::ast::{self, Type, TypeDef};
use super::VarEnv;
use crate::types::{self, Array, Entity, EntityType};

type TypeDefs = HashMap<String, TypeDef>;
type VarDefs = IndexMap<String, ast::Variable>;

const STD_TYPES: &[(&str, EntityType)] = &[
    ("String", EntityType::String),
    ("Integer", EntityType::Integer),
    ("PhoneNumber", EntityType::PhoneNumber),
    ("Date", EntityType::Date),
    ("Place", EntityType::Place),
];

pub fn resolve(module: ast::Module) -> Result<VarEnv, String> {
    let ast::Module {
        type_defs,
        variables,
    } = module;

    let mut resolved: HashMap<String, EntityType> = STD_TYPES
        .iter()
        .cloned()
        .map(|(name, ty)| (name.to_string(), ty))
        .collect();

    let type_defs = match find_type_decl_dups(type_defs, &resolved) {
        Ok(defs) => defs,
        Err(dup) => {
            return Err(format!("Duplicate type definition: {dup}"));
        },
    };

    let var_defs =
        find_var_dups(variables).map_err(|e| format!("Duplicate variable definition: {e}"))?;
    let vars: VarEnv =
        var_defs
            .into_values()
            .try_fold(IndexMap::new(), |mut map: VarEnv, var| {
                map.insert(var.name.clone(), types::Var {
                    name: var.name,
                    comment: var.comment,
                    ty: resolve_type(&var.ty, &type_defs, &mut HashSet::new(), &mut resolved)?,
                });

                Ok::<VarEnv, String>(map)
            })?;

    Ok(vars)
}

#[allow(clippy::too_many_lines)]
fn resolve_type(
    ty: &Type,
    type_defs: &TypeDefs,
    visited: &mut HashSet<String>,
    resolved: &mut HashMap<String, EntityType>,
) -> Result<Entity, String> {
    let Type {
        is_required,
        is_array,
        name,
    } = ty;
    let is_required = *is_required;
    let is_array = *is_array;

    if let Some(entity_type) = resolved.get(name) {
        let entity_type = if is_array {
            EntityType::Array(Array {
                ty: Box::new(entity_type.clone()),
            })
        } else {
            entity_type.clone()
        };

        return Ok(Entity {
            ty: entity_type,
            is_required,
        });
    }

    let def = match type_defs.get(name) {
        None => return Err(format!("Type definition not found: {name}")),
        Some(def) => def,
    };

    if !visited.insert(def.name().to_string()) {
        return Err(format!("Recursion detected: {}", def.name()));
    }

    let entity = match def {
        TypeDef::Struct(s) => {
            let ast::Struct {
                comment,
                name,
                fields,
                parent,
            } = s;

            let parent = {
                match parent
                    .as_ref()
                    .map(|p| {
                        #[allow(clippy::option_if_let_else)]
                        match resolved.get(p) {
                            Some(entity) => Ok(entity.clone()),
                            None => {
                                resolve_type(
                                    &Type {
                                        name: p.clone(),
                                        is_array: false,
                                        is_required: false,
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
                            EntityType::Struct(s) => Some(Box::new(s)),
                            _ => return Err("Parent type must be a struct.".to_string()),
                        }
                    },
                    None => None,
                }
            };

            let fields = fields
                .iter()
                .map(|f| {
                    let ast::Field { comment, name, ty } = f;
                    let entity = resolve_type(ty, type_defs, visited, resolved)?;
                    Ok(types::Field {
                        name: name.clone(),
                        comment: comment.clone(),
                        entity,
                    })
                })
                .collect::<Result<types::Fields, String>>()?;

            let struct_type = EntityType::Struct(types::Struct {
                name: name.clone(),
                comment: comment.clone(),
                fields,
                parent,
            });
            resolved.insert(name.clone(), struct_type.clone());
            Entity {
                ty: struct_type,
                is_required,
            }
        },
        TypeDef::Enum(e) => {
            let ast::Enum {
                name,
                comment,
                variants,
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
                    // TODO: check that there is no dup variants
                    let variant = types::EnumVariant {
                        name: name.clone(),
                        comment: comment.clone(),
                        field,
                    };

                    Ok(variant)
                })
                .collect::<Result<Vec<types::EnumVariant>, String>>()?;

            let enum_type = EntityType::Enum(types::Enum {
                name: name.clone(),
                comment: comment.clone(),
                variants,
            });
            resolved.insert(name.clone(), enum_type.clone());
            Entity {
                ty: enum_type,
                is_required,
            }
        },
    };

    if is_array {
        Ok(Entity {
            ty: EntityType::Array(Array {
                ty: Box::new(entity.ty),
            }),
            is_required,
        })
    } else {
        Ok(entity)
    }
}

fn find_type_decl_dups(
    type_defs: Vec<TypeDef>,
    resolved: &HashMap<String, EntityType>,
) -> Result<TypeDefs, String> {
    type_defs
        .into_iter()
        .try_fold(HashMap::new(), |mut defs, type_def| {
            let type_def_key = type_def.name().to_string();

            if resolved.contains_key(&type_def_key) {
                return Err(type_def_key);
            }

            match defs.insert(type_def_key.clone(), type_def) {
                None => Ok(defs),
                Some(_) => Err(type_def_key),
            }
        })
}

fn find_var_dups(vars: Vec<ast::Variable>) -> Result<VarDefs, String> {
    vars.into_iter().try_fold(IndexMap::new(), |mut defs, var| {
        let var_key = var.name.clone();

        match defs.insert(var_key.clone(), var) {
            None => Ok(defs),
            Some(_) => Err(var_key),
        }
    })
}

#[cfg(test)]
mod tests {
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
                }),
                TypeDef::Struct(ast::Struct {
                    name: "B".to_string(),
                    comment: None,
                    fields: vec![ast::Field {
                        name: "a".to_string(),
                        comment: "Some comment".to_string(),
                        ty: Type {
                            name: "A".to_string(),
                            is_array: false,
                            is_required: false,
                        },
                    }],
                    parent: Some("A".to_string()),
                }),
            ],
            variables: vec![ast::Variable {
                name: "a".to_string(),
                comment: "Some comment".to_string(),
                ty: Type {
                    name: "B".to_string(),
                    is_array: false,
                    is_required: false,
                },
            }],
        };

        let module = resolve(ast_module).unwrap();
        let test = {
            let a_struct = types::Struct {
                name: "A".to_string(),
                comment: None,
                fields: types::Fields::new(),
                parent: None,
            };
            let a_entity = Entity {
                ty: EntityType::Struct(a_struct.clone()),
                is_required: false,
            };

            let mut map = IndexMap::new();
            map.insert("a".to_string(), types::Var {
                name: "a".to_string(),
                comment: "Some comment".to_string(),
                ty: Entity {
                    ty: EntityType::Struct(types::Struct {
                        name: "B".to_string(),
                        comment: None,
                        fields: vec![types::Field {
                            name: "a".to_string(),
                            comment: "Some comment".to_string(),
                            entity: a_entity,
                        }],
                        parent: Some(Box::new(a_struct)),
                    }),
                    is_required: false,
                },
            });
            map
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
            })],
            variables: vec![ast::Variable {
                name: "a".to_string(),
                comment: "Some comment".to_string(),
                ty: Type {
                    name: "A".to_string(),
                    is_array: false,
                    is_required: false,
                },
            }],
        };

        let module = resolve(ast_module);
        assert_eq!(module, Err("Recursion detected: A".to_string()));
    }

    #[test]
    fn dup_type_defs() {
        let ast_module = ast::Module {
            type_defs: vec![
                TypeDef::Struct(ast::Struct {
                    name: "A".to_string(),
                    comment: None,
                    fields: vec![],
                    parent: None,
                }),
                TypeDef::Struct(ast::Struct {
                    name: "A".to_string(),
                    comment: None,
                    fields: vec![],
                    parent: None,
                }),
            ],
            variables: vec![],
        };

        let module = resolve(ast_module);
        assert_eq!(module, Err("Duplicate type definition: A".to_string()));
    }

    #[test]
    fn dup_var_defs() {
        let ast_module = ast::Module {
            type_defs: vec![],
            variables: vec![
                ast::Variable {
                    name: "a".to_string(),
                    comment: "Some comment".to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    },
                },
                ast::Variable {
                    name: "a".to_string(),
                    comment: "Some comment".to_string(),
                    ty: Type {
                        name: "String".to_string(),
                        is_array: false,
                        is_required: false,
                    },
                },
            ],
        };

        let module = resolve(ast_module);
        assert_eq!(module, Err("Duplicate variable definition: a".to_string()));
    }

    #[test]
    fn redefining_builtin_type_test() {
        let ast_module = ast::Module {
            type_defs: vec![TypeDef::Struct(ast::Struct {
                name: "String".to_string(),
                comment: None,
                fields: vec![],
                parent: None,
            })],
            variables: vec![],
        };

        let module = resolve(ast_module);
        assert_eq!(module, Err("Duplicate type definition: String".to_string()));
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn array_test() {
        let ast_module = ast::Module {
            type_defs: vec![
                TypeDef::Struct(ast::Struct {
                    name: "Person".to_string(),
                    comment: Some("Person".to_string()),
                    fields: vec![
                        ast::Field {
                            name: "kind".to_string(),
                            comment: "Person kind".to_string(),
                            ty: Type {
                                name: "PersonKind".to_string(),
                                is_array: false,
                                is_required: true,
                            },
                        },
                        ast::Field {
                            name: "field".to_string(),
                            comment: "Field with array".to_string(),
                            ty: Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: false,
                            },
                        },
                    ],
                    parent: None,
                }),
                TypeDef::Enum(ast::Enum {
                    comment: Some("Person kind".to_string()),
                    name: "PersonKind".to_string(),
                    variants: vec![
                        ast::EnumVariant {
                            name: "Newbie".to_string(),
                            comment: "Newbie".to_string(),
                            field: Some(Type {
                                name: "NewbieInfo".to_string(),
                                is_array: false,
                                is_required: true,
                            }),
                        },
                        ast::EnumVariant {
                            name: "Lawyer".to_string(),
                            comment: "Lawyer with a names".to_string(),
                            field: Some(Type {
                                name: "String".to_string(),
                                is_array: true,
                                is_required: true,
                            }),
                        },
                    ],
                }),
                TypeDef::Struct(ast::Struct {
                    comment: Some("Newbie info".to_string()),
                    name: "NewbieInfo".to_string(),
                    parent: None,
                    fields: vec![ast::Field {
                        name: "field".to_string(),
                        comment: "Newbie names".to_string(),
                        ty: Type {
                            name: "String".to_string(),
                            is_array: true,
                            is_required: true,
                        },
                    }],
                }),
            ],
            variables: vec![ast::Variable {
                name: "var".to_string(),
                comment: "Variable comment".to_string(),
                ty: Type {
                    name: "Person".to_string(),
                    is_array: true,
                    is_required: true,
                },
            }],
        };

        let module = resolve(ast_module).unwrap();

        let test = {
            let newbie_info = types::Struct {
                name: "NewbieInfo".to_string(),
                comment: Some("Newbie info".to_string()),
                fields: vec![types::Field {
                    name: "field".to_string(),
                    comment: "Newbie names".to_string(),
                    entity: Entity {
                        ty: EntityType::Array(Array {
                            ty: Box::new(EntityType::String),
                        }),
                        is_required: true,
                    },
                }],
                parent: None,
            };
            let newbie_info_entity = Entity {
                ty: EntityType::Struct(newbie_info),
                is_required: true,
            };

            let person_kind = types::Enum {
                name: "PersonKind".to_string(),
                comment: Some("Person kind".to_string()),
                variants: vec![
                    types::EnumVariant {
                        name: "Newbie".to_string(),
                        comment: "Newbie".to_string(),
                        field: Some(newbie_info_entity),
                    },
                    types::EnumVariant {
                        name: "Lawyer".to_string(),
                        comment: "Lawyer with a names".to_string(),
                        field: Some(Entity {
                            ty: EntityType::Array(Array {
                                ty: Box::new(EntityType::String),
                            }),
                            is_required: true,
                        }),
                    }
                ],
            };

            let person = types::Struct {
                name: "Person".to_string(),
                comment: Some("Person".to_string()),
                fields: vec![
                    types::Field {
                        name: "kind".to_string(),
                        comment: "Person kind".to_string(),
                        entity: Entity {
                            ty: EntityType::Enum(person_kind),
                            is_required: true,
                        },
                    },
                    types::Field {
                        name: "field".to_string(),
                        comment: "Field with array".to_string(),
                        entity: Entity {
                            ty: EntityType::Array(Array {
                                ty: Box::new(EntityType::String),
                            }),
                            is_required: false,
                        },
                    },
                ],
                parent: None,
            };

            let mut map = IndexMap::new();
            map.insert("var".to_string(), types::Var {
                name: "var".to_string(),
                comment: "Variable comment".to_string(),
                ty: Entity {
                    ty: EntityType::Array(Array {
                        ty: Box::new(EntityType::Struct(person)),
                    }),
                    is_required: true,
                },
            });
            map
        };

        assert_eq!(module, test);
    }
}
