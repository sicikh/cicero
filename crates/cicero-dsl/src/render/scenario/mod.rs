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

use context::Context;
use minijinja::Environment;

use crate::data;
use crate::types::{self, ScenarioMeta};

pub mod context;

/// A single running scenario.
///
/// Scenario is a sequence of template steps, which will be individually
/// sequentially executed.
///
/// The scenario is a state machine, which is driven by the `current_step`
/// field.
pub struct Scenario {
    /// Scenario metadata, which is stored in the individual file.
    pub(crate) meta: ScenarioMeta,
    /// Runtime context, which is used to store the variables and the state of
    /// the scenario.
    pub(crate) context: Context,
    // NB: invariant: template.steps > 0
    /// The template, which is used to render the scenario.
    pub(crate) template: Template,
    // NB: invariant: current_step < template.steps.len()
    /// The current step of the scenario.
    pub(crate) current_step: usize,
}

impl Scenario {
    pub fn new(meta: ScenarioMeta, template: Template) -> Result<Self, String> {
        if template.steps.is_empty() {
            return Err("Template has no steps".to_string());
        }

        Ok(Scenario {
            meta,
            context: Context::new(),
            template,
            current_step: 0,
        })
    }

    pub fn current_step(&self) -> usize {
        self.current_step
    }

    pub fn meta(&self) -> &ScenarioMeta {
        &self.meta
    }

    /// Returns `false` if we can continue scenario.
    pub fn is_ended(&self) -> bool {
        self.current_step >= self.template.steps.len()
    }

    /// Returns types needed for the current step.
    pub fn current_step_types(&self) -> &[types::Var] {
        &self.template.steps[self.current_step].variables
    }

    fn is_at_first_step(&self) -> bool {
        self.current_step == 0
    }

    pub fn template_at_step(&self, step: usize) -> String {
        let mut rendered = self.template.beginning_clause.clone();

        // FIXME: only for test passing before refactor
        let step = &self.template.steps[step.saturating_sub(1)];

        rendered.push_str(&step.body);

        rendered.push_str(&self.template.ending_clause);

        rendered
    }

    // TODO: refactor
    pub fn next_step(&mut self, data: HashMap<String, data::Var>) -> Result<usize, String> {
        if self.is_ended() {
            return Err("Scenario is ended".to_string());
        }

        check_data_validity(&data, self.current_step_types())?;

        self.context.insert_layer(data);

        self.current_step += 1;

        Ok(self.current_step)
    }

    pub fn step_back(&mut self) -> Result<usize, String> {
        if self.is_at_first_step() {
            return Err("Scenario is at the beginning".to_string());
        }

        self.context.drop_layer();

        self.current_step -= 1;

        Ok(self.current_step)
    }

    #[inline(always)]
    pub fn has_step_data(&self, step: usize) -> bool {
        self.context.has_layer(step)
    }

    #[inline(always)]
    pub fn has_current_step_data(&self) -> bool {
        self.has_step_data(self.current_step)
    }

    pub fn render_current_step(&self) -> Result<String, String> {
        let source = &self.current_step_template();

        let mut env = Environment::new();
        env.add_template("template", source)
            .map_err(|e| e.to_string())?;
        let template = env.get_template("template").map_err(|e| e.to_string())?;

        let rendered = template
            .render(minijinja::Value::from_struct_object(self.context.clone()))
            .map_err(|e| e.to_string())?;

        Ok(rendered)
    }

    pub fn current_step_template(&self) -> String {
        self.template_at_step(self.current_step)
    }

    pub fn full_step_template(&self) -> String {
        let mut rendered = self.template.beginning_clause.clone();

        for i in 0..self.current_step {
            let step = &self.template.steps[i];

            rendered.push_str(&step.body);
        }

        rendered.push_str(&self.template.ending_clause);

        rendered
    }
}

/// A template, which is used to render the scenario.
///
/// The template is a sequence of steps, which will be individually sequentially
/// rendered.
///
/// Template has a beginning and ending clauses, which are added to the step
/// body to properly render it.
pub struct Template {
    /// The beginning clause of the template.
    pub beginning_clause: String,
    /// The steps of the template.
    pub steps: Vec<Step>,
    /// The ending clause of the template.
    pub ending_clause: String,
}

/// A single step of the template, containing instructions for the template
/// engine.
///
/// A step may be a first phase step, which is used to initialize the scenario,
/// i. e. without producing any visual output.
pub struct Step {
    /// The name of the step.
    pub name: String,
    /// The comment of the step.
    pub comment: Option<String>,
    /// The variables of the step.
    pub variables: Vec<types::Var>,
    /// The body of the step, containing instructions to the template engine.
    pub body: String,
}

/// Checks the validity of the data.
fn check_data_validity(
    data: &HashMap<String, data::Var>,
    vars: &[types::Var],
) -> Result<(), String> {
    // Required variables are present in the data
    for var in vars.iter().filter(|&var| var.ty.is_required) {
        match (var, data.get(var.name.as_str())) {
            (
                types::Var {
                    ty: entity, name, ..
                },
                Some(data_var),
            ) => {
                if !data_var.data.is_type(&entity.ty) {
                    return Err(format!(
                        "Variable `{}` is present, but has a different type",
                        name
                    ));
                }
            },
            (types::Var { name, .. }, None) => {
                return Err(format!("Variable `{}` is required, but not present", name));
            },
        }
    }

    let vars: HashMap<&str, &types::Var> =
        vars.iter().map(|var| (var.name.as_str(), var)).collect();

    // All variables in the data are defined
    for data_var in data.values() {
        if !vars.contains_key(data_var.name.as_str()) {
            return Err(format!(
                "Variable `{}` is present, but not defined",
                data_var.name
            ));
        }
    }

    Ok(())
}
