use serde::{Deserialize, Serialize};

use crate::models::{categories, templates, users};
use crate::views::category::CategoryResponse;
use crate::views::user::UserResponse;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateWithCategoriesResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub author: UserResponse,
    pub publicity: PublicityResponse,
    pub categories: Vec<CategoryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicityResponse {
    Public,
    Private { viewers: Vec<UserResponse> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateResponse {
    pub id: i32,
}

impl TemplateWithCategoriesResponse {
    #[must_use]
    pub fn new(
        template: &templates::Model,
        author: &users::Model,
        categories: &[categories::Model],
        viewers: Option<&Vec<users::Model>>,
    ) -> Self {
        assert_eq!(template.user_id, author.id);
        let publicity = match viewers {
            Some(users) => {
                assert!(
                    !template.is_public,
                    "Template is public, but it has a list of users that it is visible to"
                );
                PublicityResponse::Private {
                    viewers: users.iter().map(UserResponse::new).collect(),
                }
            },
            None => {
                assert!(
                    template.is_public,
                    "Template is private, but there is no list of users that it is visible to"
                );

                PublicityResponse::Public
            },
        };

        Self {
            id: template.id,
            name: template.name.clone(),
            description: template.description.clone(),
            author: UserResponse::new(author),
            publicity,
            categories: categories.iter().map(CategoryResponse::new).collect(),
        }
    }
}

impl CreateTemplateResponse {
    #[must_use]
    pub fn new(template: &templates::Model) -> Self {
        Self { id: template.id }
    }
}
