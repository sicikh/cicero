//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    #[sea_orm(unique)]
    pub api_key: String,
    pub name: String,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<DateTimeWithTimeZone>,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::categories::Entity")]
    Categories,
    #[sea_orm(has_many = "super::templates::Entity")]
    Templates,
    #[sea_orm(has_many = "super::users_visible_templates::Entity")]
    UsersVisibleTemplates,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::users_visible_templates::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersVisibleTemplates.def()
    }
}

impl Related<super::templates::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_visible_templates::Relation::Templates.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::users_visible_templates::Relation::Users.def().rev())
    }
}
