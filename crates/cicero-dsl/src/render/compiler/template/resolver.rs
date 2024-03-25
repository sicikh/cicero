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

use types::context::VarEnv;

use super::ast;
use crate::render::scenario as types;

pub fn resolve_template(
    template: ast::Template,
    var_env: &VarEnv,
) -> Result<types::Template, String> {
    let ast::Template {
        beginning_clause,
        steps,
        end_clause: ending_clause,
    } = template;

    let steps = steps
        .into_iter()
        .map(|step| resolve_step(step, var_env))
        .collect::<Result<Vec<_>, String>>()?;

    check_steps(&steps)?;

    let template = types::Template {
        beginning_clause,
        steps,
        ending_clause,
    };

    Ok(template)
}

fn resolve_step(step: ast::Step, var_env: &VarEnv) -> Result<types::Step, String> {
    let ast::Step {
        name,
        comment,
        variables,
        body,
    } = step;

    let variables = variables
        .into_iter()
        .map(|var| {
            var_env
                .get(&var)
                .cloned()
                .ok_or(format!("Variable {} not found", var))
        })
        .collect::<Result<Vec<_>, String>>()?;

    let step = types::Step {
        name,
        comment,
        variables,
        body,
    };

    Ok(step)
}

fn check_steps(steps: &[types::Step]) -> Result<(), String> {
    let mut var_env = VarEnv::new();

    if steps.is_empty() {
        return Err("No steps defined".to_string());
    }

    // Check for variables that are defined in multiple steps
    for step in steps {
        for var in &step.variables {
            if var_env.contains_key(&var.name) {
                return Err(format!("Variable {} already defined", var.name));
            }

            var_env.insert(var.name.clone(), var.clone());
        }
    }

    Ok(())
}
