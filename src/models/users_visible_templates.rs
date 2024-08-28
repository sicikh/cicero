use sea_orm::entity::prelude::*;

pub use super::_entities::users_visible_templates::{self, ActiveModel, Entity, Model};

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
