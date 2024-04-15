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
pub mod context;
pub mod scenario;

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use std::collections::HashMap;
    use std::fs;

    use data::Data;
    use tempfile::tempdir;

    use crate::data;
    use crate::render::compiler::compile_scenario;
    use crate::scenario::error::ScenarioError;
    use crate::scenario::Scenario;

    fn temp_write(types_source: &str, template_source: &str, meta_source: &str) -> Scenario {
        let dir = tempdir().unwrap();

        fs::write(dir.path().join("types.cicero"), types_source).unwrap();
        fs::write(dir.path().join("template.tex.j2"), template_source).unwrap();
        fs::write(dir.path().join("meta.toml"), meta_source).unwrap();
        fs::write(dir.path().join("reference.docx"), "").unwrap();

        let scenario = compile_scenario(dir.path()).unwrap();
        dir.close().unwrap();
        scenario
    }

    #[test]
    fn render_test() {
        let types_source = r#"
            struct User {
                /// Test
                name: String,
            }
            /// Test
            let user: User;
        "#;
        let template_source = r#"\documentclass{article}
\begin{document}
%%begin
%%(Greeting)> Enter user name
%%> { user }
Hello, {{ user.name }}!
%%end
\end{document}"#;
        let meta_source = r#"
        id = 0
        name = "test"
        description = "test"
        category = "test"
        "#;

        let mut scenario = temp_write(types_source, template_source, meta_source);

        let data = HashMap::from([("user".to_string(), data::Var {
            name: "user".to_string(),
            data: Data::Struct(data::Struct {
                name: "User".to_string(),
                fields: {
                    let mut fields = HashMap::new();
                    fields.insert("name".to_string(), Data::String("Lawyer".to_string()));
                    fields
                },
            }),
        })]);

        scenario.insert_data(data).unwrap();
        let res = scenario.render().unwrap();
        let test = r#"\documentclass{article}
\begin{document}
Hello, Lawyer!
\end{document}"#;

        assert_eq!(res, test);
    }

    #[test]
    fn invalid_data() {
        let types_source = r#"
            struct User {
                /// Test
                name: String,
            }
            /// Test
            let user: User;
        "#;
        let template_source = r#"\documentclass{article}
\begin{document}
%%begin
%%(Greeting)> Enter user name
%%> { user }
Hello, {{ user.name }}!
%%end
\end{document}"#;
        let meta_source = r#"
        id = 0
        name = "test"
        description = "test"
        category = "test"
        "#;

        let mut scenario = temp_write(types_source, template_source, meta_source);

        let data = HashMap::from([("user".to_string(), data::Var {
            name: "user".to_string(),
            data: Data::Struct(data::Struct {
                name: "User".to_string(),
                fields: {
                    let mut fields = HashMap::new();
                    fields.insert(
                        "name".to_string(),
                        Data::Enum(data::Enum {
                            name: "User".to_string(),
                            discriminant: "name".to_string(),
                            field: None,
                        }),
                    );
                    fields
                },
            }),
        })]);

        let res = scenario.insert_data(data);

        assert_matches!(res, Err(ScenarioError::StepNotValid(_)));
    }
}
