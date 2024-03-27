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
use std::sync::Arc;

use minijinja::value::{Object, ObjectKind, SeqObject, StructObject, Value};
use minijinja::State;

use crate::data::{self, Data};
use crate::types::{self};

pub type VarEnv = HashMap<String, types::Var>;

#[derive(Debug, Clone, Default)]
pub struct Context {
    layers: Vec<Arc<HashMap<String, data::Var>>>,
}

impl Context {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            layers: Vec::with_capacity(capacity),
        }
    }

    pub fn get_var(&self, var_name: &str) -> Option<&data::Var> {
        self.layers.iter().rev().find_map(|env| env.get(var_name))
    }

    /// Insert data into a new layer.
    pub fn insert_layer(&mut self, data: HashMap<String, data::Var>) {
        self.layers.push(Arc::new(data));
    }

    pub fn insert(&mut self, layer: usize, data: HashMap<String, data::Var>) -> Option<()> {
        *self.layers.get_mut(layer)? = Arc::new(data);
        Some(())
    }

    #[inline(always)]
    pub fn layers(&self) -> usize {
        self.layers.len()
    }

    #[inline(always)]
    pub fn has_layer(&self, layer: usize) -> bool {
        self.layers.len() > layer
    }

    #[inline(always)]
    pub fn drop_layer(&mut self) -> Option<Arc<HashMap<String, data::Var>>> {
        self.layers.pop()
    }
}

impl StructObject for Context {
    fn get_field(&self, name: &str) -> Option<Value> {
        let var = self.get_var(name)?;
        let value = Value::from_object(var.data.clone());
        Some(value)
    }

    fn fields(&self) -> Vec<Arc<str>> {
        self.layers
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
            Data::Array(array) => ObjectKind::Seq(array),
            Data::String(_) => ObjectKind::Plain,
        }
    }

    // NB: (this cost me a lot of time)
    //  Minijinja uses this to call method on a value, but on first glance
    //  it seemed that it would cast value to a struct (as defined in kind()) and
    // call method on it.
    fn call_method(
        &self,
        state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, minijinja::Error> {
        match self {
            Data::Struct(structure) => structure.call_method(state, name, args),
            Data::Enum(enumeration) => enumeration.call_method(state, name, args),
            _ => {
                Err(minijinja::Error::new(
                    minijinja::ErrorKind::UnknownMethod,
                    "object does not have methods",
                ))
            },
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
}

impl StructObject for data::Enum {
    /// Tests, whether the field is the discriminant.
    ///
    /// Used in if statements to safely extract the field lately by calling
    /// [method with the discriminant name](data::Enum::call_method).
    fn get_field(&self, name: &str) -> Option<Value> {
        if name == self.discriminant {
            Some(Value::from_serializable(&true))
        } else {
            None
        }
    }

    fn fields(&self) -> Vec<Arc<str>> {
        vec![Arc::from(self.discriminant.as_str())]
    }

    fn field_count(&self) -> usize {
        1
    }
}

impl Object for data::Enum {
    fn kind(&self) -> ObjectKind<'_> {
        ObjectKind::Struct(self)
    }

    /// Extract the field by calling the method with the discriminant name
    /// or call enum method.
    ///
    /// Used after testing the field with [`get_field`](data::Enum::get_field).
    fn call_method(
        &self,
        _state: &State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, minijinja::Error> {
        if !args.is_empty() {
            return Err(minijinja::Error::new(
                minijinja::ErrorKind::TooManyArguments,
                "Methods does not accept arguments",
            ));
        }

        if name == self.discriminant {
            return match &self.field {
                Some(data) => Ok(Value::from_object(*data.clone())),
                None => {
                    Err(minijinja::Error::new(
                        minijinja::ErrorKind::InvalidOperation,
                        format!("Enum {} has no field", self.name),
                    ))
                },
            };
        }

        Err(minijinja::Error::new(
            minijinja::ErrorKind::UnknownMethod,
            "Method not found",
        ))
    }
}

impl SeqObject for data::Array {
    #[inline(always)]
    fn get_item(&self, idx: usize) -> Option<Value> {
        self.inner.get(idx).cloned().map(Value::from_object)
    }

    #[inline(always)]
    fn item_count(&self) -> usize {
        self.inner.len()
    }
}

impl From<Context> for Value {
    fn from(value: Context) -> Self {
        Value::from_struct_object(value)
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
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Struct(user_struct),
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), user)]))],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }

    #[test]
    fn enum_field() {
        let mut env = Environment::new();
        env.add_template(
            "test",
            "Hello, {{ user.Name() if user.Name else \"Test\" }}!",
        )
        .unwrap();
        let template = env.get_template("test").unwrap();

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "Name".to_string(),
            field: Some(Box::new(Data::String("Lawyer".to_string()))),
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Enum(user_enum),
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), user)]))],
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
        };
        let user = data::Var {
            name: "user".to_string(),
            data: Data::Enum(user_enum),
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), user)]))],
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
            layers: vec![Arc::new(HashMap::from([("user".to_string(), user)]))],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }

    #[test]
    fn enum_if_statement() {
        let src = r"{% if user.Lawyer -%}
Hello, lawyer {{ user.Lawyer().name }}!
{% elif user.Newbie -%}
Hello, user {{ user.Newbie().name }}!
{% else -%}
Hello, World!
{% endif %}";
        let mut env = Environment::new();
        env.add_template("test", src).unwrap();
        let template = env.get_template("test").unwrap();

        let person_struct = data::Struct {
            name: "Person".to_string(),
            fields: {
                let mut fields = HashMap::new();
                fields.insert("name".to_string(), Data::String("David".to_string()));
                fields
            },
        };

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "Lawyer".to_string(),
            field: Some(Box::new(Data::Struct(person_struct))),
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), data::Var {
                name: "user".to_string(),
                data: Data::Enum(user_enum),
            })]))],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, lawyer David!\n";
        assert_eq!(result, test);

        let person_struct = data::Struct {
            name: "Newbie".to_string(),
            fields: {
                let mut fields = HashMap::new();
                fields.insert("name".to_string(), Data::String("John".to_string()));
                fields
            },
        };

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "Newbie".to_string(),
            field: Some(Box::new(Data::Struct(person_struct))),
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), data::Var {
                name: "user".to_string(),
                data: Data::Enum(user_enum),
            })]))],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, user John!\n";
        assert_eq!(result, test);

        let user_enum = data::Enum {
            name: "User".to_string(),
            discriminant: "None".to_string(),
            field: None,
        };

        let context = Context {
            layers: vec![Arc::new(HashMap::from([("user".to_string(), data::Var {
                name: "user".to_string(),
                data: Data::Enum(user_enum),
            })]))],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, World!\n";
        assert_eq!(result, test);
    }
}
