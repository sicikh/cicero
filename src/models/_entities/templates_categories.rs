//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "templates_categories")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key, auto_increment = false)]
    pub template_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Categories,
    #[sea_orm(
        belongs_to = "super::templates::Entity",
        from = "Column::TemplateId",
        to = "super::templates::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Templates,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Templates.def()
    }
}
