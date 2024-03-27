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

pub use scenario::context;

pub mod compiler;
pub mod scenario;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;

    use tempfile::tempdir;

    use crate::data;
    use crate::render::compiler::compile_scenario;

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
        let meta_source = r#"
        id = 0
        name = "test"
        description = "test"
        category = "test"
        "#;

        let dir = tempdir().unwrap();

        fs::write(dir.path().join("types.cicero"), types_source).unwrap();
        fs::write(dir.path().join("template.tex.j2"), template_source).unwrap();
        fs::write(dir.path().join("meta.toml"), meta_source).unwrap();

        let mut scenario = compile_scenario(dir.path()).unwrap();
        dir.close().unwrap();

        let data = HashMap::from([("user".to_string(), data::Var {
            name: "user".to_string(),
            data: data::Data::Struct(data::Struct {
                name: "User".to_string(),
                fields: {
                    let mut fields = HashMap::new();
                    fields.insert("name".to_string(), data::Data::String("Lawyer".to_string()));
                    fields
                },
            }),
        })]);

        scenario.next_step(data).unwrap();
        let res = scenario.render_current_step().unwrap();
        let test = "beninging\n\nHello, Lawyer!\n\nending";

        assert_eq!(res, test);
    }
}
