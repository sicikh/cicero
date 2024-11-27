use serde::{Deserialize, Serialize};

use crate::models::{categories, templates, users};
use crate::views::category::Response as CategoryResponse;
use crate::views::user::Response;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithCategoriesResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub author: Response,
    pub publicity: PublicityResponse,
    pub categories: Vec<CategoryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PublicityResponse {
    Public,
    Private { viewers: Vec<Response> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    pub id: i32,
}

impl WithCategoriesResponse {
    /// # Panics
    ///
    /// Panics in the following cases:
    ///
    /// 1. if the template is public and has initialized viewers,
    ///
    /// 2. if the template is private and doesn't have initialized viewers.
    #[must_use]
    pub fn new(
        template: &templates::Model,
        author: &users::Model,
        categories: &[categories::Model],
        viewers: Option<&Vec<users::Model>>,
    ) -> Self {
        assert_eq!(template.user_id, author.id);
        let publicity = viewers.map_or_else(
            || {
                assert!(
                    template.is_public,
                    "Template is private, but there is no list of users that it is visible to"
                );

                PublicityResponse::Public
            },
            |users| {
                assert!(
                    !template.is_public,
                    "Template is public, but it has a list of users that it is visible to"
                );
                PublicityResponse::Private {
                    viewers: users.iter().map(Response::new).collect(),
                }
            },
        );

        Self {
            id: template.id,
            name: template.name.clone(),
            description: template.description.clone(),
            author: Response::new(author),
            publicity,
            categories: categories.iter().map(CategoryResponse::new).collect(),
        }
    }
}

impl CreateResponse {
    #[must_use]
    pub const fn new(template: &templates::Model) -> Self {
        Self { id: template.id }
    }
}
