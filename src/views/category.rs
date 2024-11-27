use serde::{Deserialize, Serialize};

use crate::models::_entities::categories;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub id: i32,
    pub name: String,
}

impl Response {
    #[must_use]
    pub fn new(category: &categories::Model) -> Self {
        Self {
            id: category.id,
            name: category.name.clone(),
        }
    }
}
