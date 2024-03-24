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
use std::ops::Deref;
use std::sync::Arc;

use minijinja::value::{Object, ObjectKind, StructObject, Value};

use crate::data::{self, Data};
use crate::types::{self, EntityType};

pub type VarEnv = HashMap<String, types::Var>;
pub type Methods = HashMap<String, data::Expr>;

#[derive(Debug, Clone, Default)]
pub struct Context {
    data_env: Vec<HashMap<String, data::Var>>,
    methods: HashMap<String, Arc<Methods>>,
}

impl Context {
    pub fn new(methods: HashMap<String, Arc<Methods>>) -> Self {
        Self {
            data_env: vec![],
            methods,
        }
    }

    pub fn get_var(&self, var_name: &str) -> Option<&data::Var> {
        self.data_env.iter().rev().find_map(|env| env.get(var_name))
    }

    /// Insert data into a new layer.
    ///
    /// Data should not contain methods, as they are added from the context.
    pub fn insert_layer(&mut self, mut data: HashMap<String, data::Var>) {
        for var in data.values_mut() {
            insert_methods(&mut var.data, &self.methods);
        }

        self.data_env.push(data);
    }

    #[inline(always)]
    pub fn has_layer(&self, layer: usize) -> bool {
        self.data_env.len() > layer
    }

    #[inline(always)]
    pub fn drop_layer(&mut self) -> Option<HashMap<String, data::Var>> {
        self.data_env.pop()
    }
}

impl StructObject for Context {
    fn get_field(&self, name: &str) -> Option<Value> {
        let var = self.get_var(name)?;
        let value = Value::from_object(var.data.clone());
        Some(value)
    }

    fn fields(&self) -> Vec<Arc<str>> {
        self.data_env
            .iter()
            .flat_map(|env| env.keys().map(|var_name| Arc::from(var_name.as_str())))
            .collect()
    }
}

impl Object for Data {
    fn kind(&self) -> ObjectKind<'_> {
        match &self {
            Data::Struct(structure) => ObjectKind::Struct(structure),
            Data::Enum(enumeration) => ObjectKind::Struct(enumeration),
            Data::String(_) => ObjectKind::Plain,
        }
    }
}

impl StructObject for data::Struct {
    fn get_field(&self, name: &str) -> Option<Value> {
        self.fields
            .get(name)
            .map(|data| Value::from_object(data.clone()))
    }

    fn fields(&self) -> Vec<Arc<str>> {
        self.fields.keys().map(|s| Arc::from(s.as_str())).collect()
    }
}

impl Object for data::Struct {
    fn kind(&self) -> ObjectKind<'_> {
        ObjectKind::Struct(self)
    }

    fn call_method(
        &self,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, minijinja::Error> {
        if !args.is_empty() {
            return Err(minijinja::Error::new(
                minijinja::ErrorKind::TooManyArguments,
                "Method does not accept arguments",
            ));
        }

        match self.methods.as_ref().and_then(|inner| inner.get(name)) {
            Some(expr) => {
                match expr.evaluate(&Data::Struct(self.clone())) {
                    Ok(value) => Ok(Value::from(value)),
                    Err(err) => {
                        Err(minijinja::Error::new(
                            minijinja::ErrorKind::InvalidOperation,
                            format!("Method failed: {}", err),
                        ))
                    },
                }
            },
            None => {
                Err(minijinja::Error::new(
                    minijinja::ErrorKind::UnknownMethod,
                    "Method not found",
                ))
            },
        }
    }
}

#[allow(clippy::map_clone)] // to show that we are cloning the Arc
fn insert_methods(data: &mut Data, methods: &HashMap<String, Arc<Methods>>) {
    match data {
        Data::Struct(structure) => {
            structure.methods = methods.get(&structure.name).map(Arc::clone);

            for field in structure.fields.values_mut() {
                insert_methods(field, methods);
            }
        },
        Data::Enum(enumeration) => {
            enumeration.methods = methods.get(&enumeration.name).map(Arc::clone);

            if let Some(field) = enumeration.field.as_mut() {
                insert_methods(field, methods);
            }
        },
        _ => {},
    }
}

impl StructObject for data::Enum {
    // TODO: as enum carries only a discriminant,
    //  we cannot test if the field is present in the enum.
    //  Think about this, maybe carry all discriminants in the enum data?
    fn get_field(&self, name: &str) -> Option<Value> {
        if name == self.discriminant {
            match self.field {
                // TODO: maybe don't use true?
                None => Some(Value::from_serializable(&true)),
                Some(ref data) => Some(Value::from_object(*data.clone())),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use minijinja::Environment;

    use super::*;

    #[test]
    fn struct_field() {
        let mut env = Environment::new();
        env.add_template("test", "Hello, {{ user.name }}!").unwrap();
        let template = env.get_template("test").unwrap();

        let user_struct = data::Struct {
            name: "User".to_string(),
            fields: {
                let mut fields = HashMap::new();
                fields.insert("name".to_string(), Data::String("Lawyer".to_string()));
                fields
            },
            methods: None,
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Struct(user_struct),
        };

        let context = Context {
            data_env: vec![HashMap::from([("user".to_string(), user)])],
            ..Default::default()
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }

    #[test]
    fn enum_field() {
        let mut env = Environment::new();
        env.add_template("test", "Hello, {{ user.Name }}!").unwrap();
        let template = env.get_template("test").unwrap();

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "Name".to_string(),
            field: Some(Box::new(Data::String("Lawyer".to_string()))),
            methods: None,
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Enum(user_enum),
        };

        let context = Context {
            data_env: vec![HashMap::from([("user".to_string(), user)])],
            ..Default::default()
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }

    #[test]
    fn enum_simple_field() {
        let mut env = Environment::new();
        env.add_template("test", "Hello{{\", Admin\" if user.is_admin }}!")
            .unwrap();
        let template = env.get_template("test").unwrap();

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "is_admin".to_string(),
            field: None,
            methods: None,
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Enum(user_enum),
        };

        let context = Context {
            data_env: vec![HashMap::from([("user".to_string(), user)])],
            ..Default::default()
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Admin!";
        assert_eq!(result, test);
    }

    #[test]
    fn string_test() {
        let mut env = Environment::new();
        env.add_template("test", "Hello, {{ user }}!").unwrap();
        let template = env.get_template("test").unwrap();

        let user = data::Var {
            name: "user".to_string(),
            data: Data::String("Lawyer".to_string()),
        };

        let context = Context {
            data_env: vec![HashMap::from([("user".to_string(), user)])],
            ..Default::default()
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }
}
