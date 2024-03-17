use types::context::VarEnv;

use super::ast;
use crate::render::scenario as types;
use crate::types::Var;

pub fn resolve_template(
    template: ast::Template,
    var_env: &VarEnv,
) -> Result<types::Template, String> {
    let ast::Template {
        beginning_clause,
        steps,
        ending_clause,
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
        is_first_phase,
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
        is_first_phase,
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

    // Check that first phase steps are at the beginning
    let mut first_phase = false;
    for step in steps {
        if step.is_first_phase {
            if !first_phase {
                first_phase = true;
            } else {
                return Err("First phase step after non-first phase step".to_string());
            }
        } else if first_phase {
            return Err("Non-first phase step after first phase step".to_string());
        }
    }

    Ok(())
}
