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

use super::ast::*;
use crate::compiler::parse_markdown;

// TODO: find more suitable way to parse templates
// FIXME: handle \r
#[allow(clippy::manual_strip)]
pub fn parse_template(input: &str) -> Result<Template, String> {
    let beginning_clause_end = input
        .find("%%begin\n")
        .ok_or("Couldn't find `%%begin` command")?;

    let beginning_clause = input[..beginning_clause_end].to_string();

    let end_clause_start = input
        .find("%%end\n")
        .ok_or("Couldn't find `%%end` command")?
        + "%%end\n".chars().count();

    let end_clause = input[end_clause_start..].to_string();

    let mut steps: Vec<Step> = Vec::new();
    let mut current_step: Option<Step> = None;

    for line in input[beginning_clause_end..end_clause_start - "%%end\n".chars().count()].lines() {
        if line.starts_with("%%(") {
            if let Some(step) = current_step {
                steps.push(step);
            }
            let parts: Vec<&str> = line.split(')').collect();
            let name = parts[0][3..].to_string();
            let mut comment = None;

            let variables = if parts[1].contains('{') {
                parts[1][3..parts[1].len() - 1]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect()
            } else {
                let mut comment_ = parts[1][2..].to_string();
                comment_.push('\n');
                comment = Some(comment_);
                Vec::new()
            };

            current_step = Some(Step {
                name,
                comment,
                variables,
                body: String::new(),
            });
        } else if line.starts_with("%%>") {
            if let Some(step) = &mut current_step {
                if line.contains('{') {
                    let vars: Vec<String> = line[5..line.len() - 2]
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect();
                    step.variables = vars;
                } else if let Some(comment) = &mut step.comment {
                    comment.push_str(line[3..].trim());
                    comment.push('\n');
                }
            }
        } else if let Some(step) = &mut current_step {
            step.body.push_str(line);
            step.body.push('\n');
        }
    }

    if let Some(step) = current_step {
        steps.push(step);
    }

    steps = steps
        .into_iter()
        .map(|mut step| {
            if let Some(comment) = &mut step.comment {
                *comment = parse_markdown(comment);
            }
            step
        })
        .collect();

    Ok(Template {
        beginning_clause,
        steps,
        end_clause,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template() {
        let input = r#"begin clause

%%begin
%%(step1)> Description
%%>
%%> another line
%%> { var1, var2 }
step1 body {{ var2 }}

%%(step2)> { var3 }
step2 body

%%(step3)> Description
%%> { var4 }

%%end
end clause"#;

        let template = parse_template(input).unwrap();
        let test = Template {
            beginning_clause: "begin clause\n\n".to_string(),
            steps: vec![
                Step {
                    name: "step1".to_string(),
                    comment: Some("<p>Description</p>\n<p>another line</p>\n".to_string()),
                    variables: vec!["var1".to_string(), "var2".to_string()],
                    body: "step1 body {{ var2 }}\n\n".to_string(),
                },
                Step {
                    name: "step2".to_string(),
                    comment: None,
                    variables: vec!["var3".to_string()],
                    body: "step2 body\n\n".to_string(),
                },
                Step {
                    name: "step3".to_string(),
                    comment: Some("<p>Description</p>\n".to_string()),
                    variables: vec!["var4".to_string()],
                    body: "\n".to_string(),
                },
            ],
            end_clause: "end clause".to_string(),
        };

        assert_eq!(template, test);
    }
}
