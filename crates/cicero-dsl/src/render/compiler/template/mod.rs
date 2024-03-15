pub struct Template {
    pub beginning: String,
    pub steps: Vec<Step>,
    pub ending: String,
}

pub struct Step {
    pub is_first_phase: bool,
    pub name: String,
    pub comment: String,
    pub variables: Vec<String>,
    pub body: String,
}

pub fn parse_template(_input: &str) -> Result<Template, String> {
    todo!()
}
