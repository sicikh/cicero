use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use minijinja::value::{Object, ObjectKind, StructObject, Value};

use crate::data::{self, Data};
use crate::types;

pub type VarEnv = HashMap<String, types::Var>;

pub struct Context {
    // TODO: arc + mutex to avoid cloning? this would cause to use tokio::sync::Mutex
    pub inner: Vec<Vec<data::Var>>,
}

impl Context {
    // TODO: optimize
    pub fn get(&self, name: &str) -> Option<&data::Var> {
        self.inner
            .iter()
            .rev()
            .find_map(|env| env.iter().find(|var| var.name == name))
    }
}

pub struct Scenario {
    pub context: Context,
    // TODO: meta, types, etc
}

impl StructObject for Context {
    fn get_field(&self, name: &str) -> Option<Value> {
        let var = self.get(name)?;
        let value = Value::from_object(var.data.clone());
        Some(value)
    }

    fn fields(&self) -> Vec<Arc<str>> {
        self.inner
            .iter()
            .flat_map(|env| env.iter().map(|var| Arc::from(var.name.as_str())))
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

        match self.methods.get(name) {
            Some(expr) => {
                match expr.evaluate(&data::Data::Struct(self.clone())) {
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

impl StructObject for data::Enum {
    // TODO: as enum carries only a discriminant,
    //  we cannot test if the field is present in the enum.
    //  Think about this, maybe carry all discriminants in the enum data?
    fn get_field(&self, name: &str) -> Option<Value> {
        if name == self.discriminant {
            match self.field {
                // FIXME: maybe don't use true?
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
    use indexmap::IndexMap;
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
                let mut fields = IndexMap::new();
                fields.insert("name".to_string(), data::Data::String("Lawyer".to_string()));
                fields
            },
            methods: Arc::new(HashMap::new()),
        };
        let user = data::Var {
            name: "user".to_string(),
            data: data::Data::Struct(user_struct),
        };

        let context = Context {
            inner: vec![vec![user]],
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
            field: Some(Box::new(data::Data::String("Lawyer".to_string()))),
            methods: Arc::new(HashMap::new()),
        };
        let user = data::Var {
            name: "user".to_string(),
            data: data::Data::Enum(user_enum),
        };

        let context = Context {
            inner: vec![vec![user]],
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
            methods: Arc::new(HashMap::new()),
        };
        let user = data::Var {
            name: "user".to_string(),
            data: data::Data::Enum(user_enum),
        };

        let context = Context {
            inner: vec![vec![user]],
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
            data: data::Data::String("Lawyer".to_string()),
        };

        let context = Context {
            inner: vec![vec![user]],
        };

        let result = template.render(Value::from_struct_object(context)).unwrap();
        let test = "Hello, Lawyer!";
        assert_eq!(result, test);
    }
}
