pub mod context;

use std::collections::{HashMap, HashSet};
use std::f32::consts::E;

use context::Context;
use minijinja::Environment;

use crate::data;
use crate::types::{self, ScenarioMeta};

/// A single running scenario.
///
/// Scenario is a sequence of template steps, which will be individually
/// sequentially executed.
///
/// The scenario is a state machine, which is driven by the `current_step`
/// field.
pub struct Scenario {
    // TODO: ScenarioMeta is not defined here because the client needs it.
    //  So think about organizing it differently.
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
    pub fn new(
        meta: ScenarioMeta,
        template: Template,
        methods: HashMap<String, std::sync::Arc<HashMap<String, data::Expr>>>,
    ) -> Result<Self, String> {
        if template.steps.is_empty() {
            return Err("Template has no steps".to_string());
        }

        let context = Context::new(methods);

        Ok(Scenario {
            meta,
            context,
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

    pub fn template(&self) -> &Template {
        &self.template
    }

    pub fn context(&self) -> &Context {
        &self.context
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

    pub fn is_in_first_phase(&self) -> bool {
        self.template.steps[self.current_step].is_first_phase
    }

    pub fn next_step(&mut self, data: Vec<data::Var>) -> Result<usize, String> {
        if self.is_ended() {
            return Err("Scenario is ended".to_string());
        }

        check_data_validity(data.as_slice(), self.current_step_types())?;

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

    pub fn render_step(&self) -> Result<String, String> {
        let rendered = self.step_template();

        let source = &self.step_template();
        let mut env = Environment::new();
        env.add_template("template", source)
            .map_err(|e| e.to_string())?;
        let template = env.get_template("template").map_err(|e| e.to_string())?;
        let rendered = template
            .render(minijinja::Value::from_struct_object(self.context.clone()))
            .map_err(|e| e.to_string())?;

        Ok(rendered)
    }

    pub fn step_template(&self) -> String {
        let mut rendered = self.template.beginning_clause.clone();

        let step = &self.template.steps[self.current_step - 1];

        rendered.push_str(&step.body);

        rendered.push_str(&self.template.ending_clause);

        rendered
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
    /// Whether the step is a first phase step.
    pub is_first_phase: bool,
    /// The name of the step.
    pub name: String,
    /// The comment of the step.
    pub comment: String,
    /// The variables of the step.
    pub variables: Vec<types::Var>,
    /// The body of the step, containing instructions to the template engine.
    pub body: String,
}

/// Checks the validity of the data.
fn check_data_validity(data: &[data::Var], types: &[types::Var]) -> Result<(), String> {
    let data: HashMap<&str, &data::Var> = data.iter().map(|var| (var.name.as_str(), var)).collect();
    let types: HashMap<&str, &types::Var> =
        types.iter().map(|var| (var.name.as_str(), var)).collect();

    for ty in types.values().filter(|ty| ty.ty.is_required) {
        match (ty, data.get(ty.name.as_str())) {
            (types::Var { ty, name, .. }, Some(var)) => {
                if !var.data.is_ty(&ty.ty) {
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

    for var in data.values() {
        if !types.contains_key(var.name.as_str()) {
            return Err(format!(
                "Variable `{}` is present, but not defined",
                var.name
            ));
        }
    }

    Ok(())
}
