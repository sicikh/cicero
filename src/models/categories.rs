use loco_rs::prelude::*;

// use sea_orm::entity::prelude::*;
pub use super::_entities::categories::{self, ActiveModel, Entity, Model};
use super::_entities::templates;

impl ActiveModelBehavior for ActiveModel {
    // extend active model below (keep comment for generators)
}

impl Model {
    /// # Errors
    ///
    /// When entity is not found
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let category = Entity::find_by_id(id).one(db).await?;
        category.ok_or_else(|| ModelError::EntityNotFound)
    }

    /// # Errors
    ///
    /// When entity is not found
    pub async fn find_for_template(
        db: &DatabaseConnection,
        template_id: i32,
    ) -> ModelResult<Vec<Self>> {
        let template = templates::Model::find_by_id(db, template_id).await?;
        let categories = template.find_related(Entity).all(db).await?;

        Ok(categories)
    }
}
