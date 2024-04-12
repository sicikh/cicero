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
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

use self::error::{Result, ScenarioError};
use super::context::Context;
use crate::data;
use crate::render::compiler::cicero::check_data_validity;
use crate::types::{self, ScenarioMeta, ScenarioStep};

pub mod error;

/// A single running scenario.
///
/// Scenario is a sequence of template steps, which will be individually
/// sequentially executed.
///
/// The scenario is a state machine, which is driven by the `current_step`
/// field.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Scenario metadata, which is stored in the individual file.
    pub(crate) meta: ScenarioMeta,
    /// Runtime context, which is used to store the variables and the state of
    /// the scenario.
    pub(crate) context: Context,
    // NB: invariant: template.steps > 0
    /// The template, which is used to render the scenario.
    pub(crate) template: Template,
    // NB: invariant: current_step < template.steps.len() && current_step <= filled_steps()
    /// The current step of the scenario.
    pub(crate) current_step: usize,
}

impl Scenario {
    pub fn new(meta: ScenarioMeta, template: Template) -> Option<Self> {
        if template.steps.is_empty() {
            return None;
        }

        Some(Self {
            meta,
            context: Context::new(),
            template,
            current_step: 0,
        })
    }

    #[inline(always)]
    pub fn current_step(&self) -> usize {
        self.current_step
    }

    #[inline(always)]
    pub fn step_data(&self, step: usize) -> Option<HashMap<String, data::Var>> {
        self.context.layers().get(step).map(|data| (**data).clone())
    }

    #[inline(always)]
    pub fn current_step_data(&self) -> Option<HashMap<String, data::Var>> {
        self.step_data(self.current_step)
    }

    #[inline(always)]
    pub fn filled_steps(&self) -> usize {
        self.context.layers_len()
    }

    #[inline(always)]
    pub fn template(&self) -> &Template {
        &self.template
    }

    #[inline(always)]
    fn has_step_data(&self, step: usize) -> bool {
        step < self.filled_steps()
    }

    #[inline(always)]
    pub fn meta(&self) -> &ScenarioMeta {
        &self.meta
    }

    /// Returns `false` if we can continue scenario.
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        debug_assert!(self.current_step < self.template.steps.len());

        self.current_step == self.template.steps.len() - 1
    }

    // TODO: find more appropriate name
    #[inline(always)]
    pub fn scenario_step(&self) -> ScenarioStep {
        let curr_step = &self.template.steps[self.current_step];
        ScenarioStep {
            name: curr_step.name.clone(),
            header: curr_step.comment.clone(),
            variables: self.current_step_types().to_vec(),
        }
    }

    // TODO: find more appropriate name
    #[inline(always)]
    pub fn scenario_steps(&self) -> Vec<ScenarioStep> {
        self.template
            .steps
            .iter()
            .map(|step| {
                ScenarioStep {
                    name: step.name.clone(),
                    header: step.comment.clone(),
                    variables: step.variables.clone(),
                }
            })
            .collect()
    }

    #[inline(always)]
    pub fn steps_names(&self) -> Vec<String> {
        self.template
            .steps
            .iter()
            .map(|step| step.name.clone())
            .collect()
    }

    /// Returns types needed for the current step.
    #[inline(always)]
    pub fn current_step_types(&self) -> &[types::Var] {
        &self.template.steps[self.current_step].variables
    }

    #[inline(always)]
    pub fn step_to(&mut self, step: usize) -> Result<()> {
        if step >= self.template.steps.len() {
            return Err(ScenarioError::StepOutOfBounds(step));
        }

        // FIXME: check
        if step > self.filled_steps() {
            return Err(ScenarioError::StepNotFilled(step));
        }

        self.current_step = step;

        Ok(())
    }

    pub fn insert_data(&mut self, data: HashMap<String, data::Var>) -> Result<()> {
        check_data_validity(&data, self.current_step_types())
            .map_err(|_| ScenarioError::StepNotValid(self.current_step))?;

        if self.current_step == self.filled_steps() {
            self.context.insert_layer(data)
        } else {
            self.context
                .insert(self.current_step, data)
                .expect("Invariants not satisfied.");
        }

        Ok(())
    }

    pub fn render(&self) -> Result<String> {
        if !self.has_step_data(self.current_step) {
            return Err(ScenarioError::StepNotFilled(self.current_step));
        }

        let source = self
            .template
            .at_step(self.current_step)
            .expect("Invariants not satisfied.");

        let mut env = minijinja::Environment::new();
        env.add_template("template", source.as_str())?;
        let template = env.get_template("template")?;

        let rendered = template.render::<minijinja::Value>(self.context.clone().into())?;
        Ok(rendered)
    }

    pub fn full_render(&self) -> Result<String> {
        if !self.has_step_data(self.current_step) {
            return Err(ScenarioError::StepNotFilled(self.current_step));
        }

        let source = self
            .template
            .up_to_step(self.current_step)
            .expect("Invariants not satisfied.");

        let mut env = minijinja::Environment::new();
        env.add_template("template", source.as_str())?;
        let template = env.get_template("template")?;

        let rendered = template.render::<minijinja::Value>(self.context.clone().into())?;
        Ok(rendered)
    }

    pub fn render_pdf(&self, dir: impl AsRef<Path>) -> Result<PathBuf> {
        let rendered = self.render()?;

        self.render_pdf_inner(rendered.as_str(), dir)
    }

    pub fn full_render_pdf(&self, dir: impl AsRef<Path>) -> Result<PathBuf> {
        let rendered = self.full_render()?;

        self.render_pdf_inner(rendered.as_str(), dir)
    }

    fn render_pdf_inner(&self, rendered: &str, dir: impl AsRef<Path>) -> Result<PathBuf> {
        let mut path = dir.as_ref().to_path_buf();
        path.push("rendered.tex");
        std::fs::write(&path, rendered).map_err(ScenarioError::FileWriteError)?;

        Command::new("tectonic")
            .args([OsStr::new("-X"), OsStr::new("compile"), path.as_os_str()])
            .spawn()
            .map_err(ScenarioError::TectonicError)?;

        Ok(path.with_extension("pdf"))
    }

    pub fn render_docx(
        &self,
        dir: impl AsRef<Path>,
        reference_path: impl AsRef<Path>,
    ) -> Result<()> {
        let rendered = self.render()?;

        self.render_docx_inner(rendered.as_str(), dir, reference_path)
    }

    pub fn full_render_docx(
        &self,
        dir: impl AsRef<Path>,
        reference_path: impl AsRef<Path>,
    ) -> Result<()> {
        let rendered = self.full_render()?;

        self.render_docx_inner(rendered.as_str(), dir, reference_path)
    }

    fn render_docx_inner(
        &self,
        rendered: &str,
        dir: impl AsRef<Path>,
        reference_path: impl AsRef<Path>,
    ) -> Result<()> {
        let mut path = dir.as_ref().to_path_buf();
        path.push("rendered.tex");
        let mut output_path = path.clone();
        output_path.set_extension("docx");
        std::fs::write(&path, rendered).map_err(ScenarioError::FileWriteError)?;
        // TODO: верные пути, понимать, откуда мы запускаемся
        Command::new("pandoc")
            .arg(path.as_os_str())
            .args([OsStr::new("-o"), output_path.as_os_str()])
            .args(["--from", "latex"])
            .args([
                OsStr::new("--reference-doc"),
                reference_path.as_ref().as_os_str(),
            ])
            .spawn()
            .map_err(ScenarioError::PandocError)?;

        Ok(())
    }
}

/// A template, which is used to render the scenario.
///
/// The template is a sequence of steps, which will be individually sequentially
/// rendered.
///
/// Template has a beginning and ending clauses, which are added to the step
/// body to properly render it.
#[derive(Debug, Clone)]
pub struct Template {
    /// The beginning clause of the template.
    pub beginning_clause: String,
    /// The steps of the template.
    pub steps: Vec<Step>,
    /// The ending clause of the template.
    pub ending_clause: String,
}

impl Template {
    /// Renders the template at the given step.
    pub fn at_step(&self, step: usize) -> Option<String> {
        let mut rendered = self.beginning_clause.clone();

        rendered.push_str(self.steps.get(step)?.body.as_str());

        rendered.push_str(self.ending_clause.as_str());

        Some(rendered)
    }

    /// Renders the template up to the given step included.
    pub fn up_to_step(&self, step: usize) -> Option<String> {
        let mut rendered = self.beginning_clause.clone();

        for i in 0..step {
            rendered.push_str(self.steps.get(i)?.body.as_str());
        }

        rendered.push_str(self.ending_clause.as_str());

        Some(rendered)
    }
}

/// A single step of the template, containing instructions for the template
/// engine.
///
/// A step may be a first phase step, which is used to initialize the scenario,
/// i. e. without producing any visual output.
#[derive(Debug, Clone)]
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
