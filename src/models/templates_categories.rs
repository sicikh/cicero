use sea_orm::entity::prelude::*;

pub use super::_entities::templates_categories::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for ActiveModel {
    // extend active model below (keep comment for generators)
}
