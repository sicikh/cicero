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

pub mod compiler;
pub mod scenario;

pub use scenario::context;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use self::context::Methods;
    use self::scenario::Scenario;
    use super::*;
    use crate::render::compiler::compile_template;
    use crate::types::ScenarioMeta;
    use crate::{data, types};

    #[test]
    fn render_test() {
        let types_source = r#"
            struct User {
                name: String,
            }

            let user: User;
        "#;
        let template_source = r#"beninging

%%begin
%%(Greeting)> Enter user name
%%> { user }
Hello, {{ user.name }}!

%%end
ending"#;

        let module = compiler::parse_module(types_source).unwrap();
        let var_env = compiler::resolve(module).unwrap();
        let method_map = HashMap::new();
        let template = compile_template(template_source, &var_env).unwrap();
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

        let data = HashMap::from([("user".to_string(), data::Var {
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
        })]);

        scenario.next_step(data).unwrap();
        let res = scenario.render_step().unwrap();
        let test = "beninging\n\nHello, Lawyer!\n\nending";

        assert_eq!(res, test);
    }
}
