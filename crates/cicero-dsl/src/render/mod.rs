pub mod compiler;
pub mod scenario;

pub use scenario::context;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use self::context::Methods;
    use self::scenario::Scenario;
    use super::*;
    use crate::types::ScenarioMeta;
    use crate::{data, types};

    #[test]
    fn render_test() {
        let step = compiler::Step {
            name: "Greeting".to_string(),
            comment: "Enter user name".to_string(),
            variables: vec!["user".to_string()],
            is_first_phase: false,
            body: "Hello, {{ user.name }}!\n".to_string(),
        };
        let mut ast_template: compiler::Template = compiler::Template {
            beginning_clause: "beninging\n".to_string(),
            steps: vec![step],
            ending_clause: "ending".to_string(),
        };

        let types_source = r#"
            struct User {
                name: String,
            }

            let user: User;
        "#;

        let module = compiler::parse_module(types_source).unwrap();
        let var_env = compiler::resolve(module).unwrap();
        let method_map = HashMap::new();
        let template = compiler::resolve_template(ast_template, &var_env).unwrap();
        let meta = ScenarioMeta {
            name: "name".to_string(),
            description: "description".to_string(),
            id: 0,
            date_of_creation: "today".to_string(),
            date_of_last_change: "today".to_string(),
            author: "me".to_string(),
        };
        let mut scenario = Scenario::new(meta, template, method_map).unwrap();
        assert_eq!(scenario.current_step_types(), &[var_env
            .get("user")
            .cloned()
            .unwrap()]);

        let data = vec![data::Var {
            name: "user".to_string(),
            data: data::Data::Struct(data::Struct {
                name: "User".to_string(),
                methods: None,
                fields: {
                    let mut fields = HashMap::new();
                    fields.insert("name".to_string(), data::Data::String("Lawyer".to_string()));
                    fields
                },
            }),
        }];

        scenario.next_step(data).unwrap();
        let res = scenario.render_step().unwrap();
        let test = r#"beninging
Hello, Lawyer!
ending"#;

        assert_eq!(res, test);
    }
}
