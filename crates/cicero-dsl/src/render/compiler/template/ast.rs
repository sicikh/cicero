pub struct Template {
    pub beginning_clause: String,
    pub steps: Vec<Step>,
    pub ending_clause: String,
}

pub struct Step {
    pub is_first_phase: bool,
    pub name: String,
    pub comment: String,
    pub variables: Vec<String>,
    pub body: String,
}
