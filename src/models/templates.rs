use std::path::PathBuf;

use cicero_dsl::compiler::compile_types;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Query;
use sea_orm::Condition;
use tokio::fs;

pub use super::_entities::templates::{self, ActiveModel, Entity, Model};
use super::_entities::{categories, templates_categories, users, users_visible_templates};
use crate::controllers::templates::{CreateTemplateParams, PublicityParams};

impl ActiveModelBehavior for templates::ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl templates::Model {
    pub async fn create(
        db: &DatabaseConnection,
        params: &CreateTemplateParams,
        author_id: i32,
        docx: &[u8],
        dsl: &str,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        compile_types(dsl).map_err(|_| ModelError::Any("Invalid DSL".into()))?;

        let author = users::Entity::find_by_id(author_id)
            .one(&txn)
            .await?
            .ok_or(ModelError::EntityNotFound)?;

        let mut categories = Vec::with_capacity(params.categories.len());

        for category_id in params.categories.iter().cloned() {
            let category = categories::Entity::find_by_id(category_id)
                .one(&txn)
                .await?
                .ok_or(ModelError::EntityNotFound)?;
            categories.push(category);
        }

        let viewers = match &params.publicity {
            PublicityParams::Public => None,
            PublicityParams::Private { viewers: visible_to } => {
                let mut viewers = Vec::with_capacity(visible_to.len());

                for viewer_email in visible_to.iter().map(String::as_str) {
                    let viewer = users::Entity::find()
                        .filter(
                            query::condition()
                                .eq(users::Column::Email, viewer_email)
                                .build(),
                        )
                        .one(&txn)
                        .await?
                        .ok_or(ModelError::EntityNotFound)?;
                    viewers.push(viewer);
                }

                Some(viewers)
            },
        };

        let template = templates::ActiveModel {
            name: Set(params.name.clone()),
            description: Set(params.description.clone()),
            is_public: Set(viewers.is_none()),
            user_id: Set(author.id),
            ..Default::default()
        };

        let template = templates::Entity::insert(template).exec(&txn).await?;
        let template_id = template.last_insert_id;

        fs::write(format!("./data/templates/{}.docx", template_id), docx)
            .await
            .map_err(|_| ModelError::Any("Error writing file".into()))?;

        fs::write(format!("./data/templates/{}.dsl", template_id), dsl)
            .await
            .map_err(|_| ModelError::Any("Error writing file".into()))?;

        for category in categories.iter() {
            let template_category = templates_categories::ActiveModel {
                template_id: Set(template_id),
                category_id: Set(category.id),
                ..Default::default()
            };

            templates_categories::Entity::insert(template_category)
                .exec(&txn)
                .await?;
        }

        if let Some(viewers) = viewers {
            for viewer in viewers.iter() {
                let viewer_template = users_visible_templates::ActiveModel {
                    user_id: Set(viewer.id),
                    template_id: Set(template_id),
                    ..Default::default()
                };

                users_visible_templates::Entity::insert(viewer_template)
                    .exec(&txn)
                    .await?;
            }
        }

        let template = templates::Entity::find_by_id(template_id)
            .one(&txn)
            .await?
            .ok_or(ModelError::EntityNotFound);

        txn.commit().await?;

        template
    }

    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let template = templates::Entity::find_by_id(id).one(db).await?;
        template.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_public(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let templates = templates::Entity::find()
            .filter(templates::Column::IsPublic.eq(true))
            .all(db)
            .await?;
        Ok(templates)
    }

    pub async fn find_visible_to_user(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> ModelResult<Vec<Self>> {
        let templates = templates::Entity::find()
            .filter(
                Condition::any()
                    .add(templates::Column::IsPublic.eq(true))
                    .add(templates::Column::UserId.eq(user_id))
                    .add(
                        templates::Column::Id.in_subquery(
                            Query::select()
                                .column(users_visible_templates::Column::TemplateId)
                                .from(users_visible_templates::Entity)
                                .and_where(users_visible_templates::Column::UserId.eq(user_id))
                                .to_owned(),
                        ),
                    ),
            )
            .all(db)
            .await?;

        Ok(templates)
    }

    pub async fn find_visible(
        db: &DatabaseConnection,
        user_id: Option<i32>,
    ) -> ModelResult<Vec<Self>> {
        match user_id {
            Some(user_id) => Self::find_visible_to_user(db, user_id).await,
            None => Self::find_public(db).await,
        }
    }

    pub async fn find_by_id_for_user(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;

        let template = Self::find_by_id(db, id).await?;
        if template.is_public || template.user_id == user_id {
            txn.commit().await?;
            return Ok(template);
        }

        let is_visible = users_visible_templates::Entity::find_by_id((id, user_id))
            .one(db)
            .await?;

        if is_visible.is_some() {
            txn.commit().await?;
            return Ok(template);
        };

        txn.commit().await?;
        Err(ModelError::Any("Unauthorized".into()))
    }

    pub async fn find_public_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let template = templates::Entity::find()
            .filter(templates::Column::IsPublic.eq(true))
            .filter(templates::Column::Id.eq(id))
            .one(db)
            .await?;
        template.ok_or_else(|| ModelError::EntityNotFound)
    }

    pub async fn find_visible_by_id(
        db: &DatabaseConnection,
        id: i32,
        user_id: Option<i32>,
    ) -> ModelResult<Self> {
        match user_id {
            Some(user_id) => Self::find_by_id_for_user(db, id, user_id).await,
            None => Self::find_public_by_id(db, id).await,
        }
    }

    pub async fn find_docx(id: i32) -> ModelResult<Vec<u8>> {
        let file_path = PathBuf::from(format!("./data/templates/{}.docx", id));

        if !file_path.exists() {
            return Err(ModelError::EntityNotFound);
        }

        let buffer = fs::read(file_path)
            .await
            .map_err(|_| ModelError::Any("Error reading file".into()))?;

        Ok(buffer)
    }

    pub async fn find_dsl(id: i32) -> ModelResult<String> {
        let file_path = PathBuf::from(format!("./data/templates/{}.dsl", id));

        if !file_path.exists() {
            return Err(ModelError::EntityNotFound);
        }

        let buffer = fs::read_to_string(file_path)
            .await
            .map_err(|_| ModelError::Any("Error reading file".into()))?;

        Ok(buffer)
    }

    pub async fn delete_template(
        db: &DatabaseConnection,
        id: i32,
        user_id: i32,
    ) -> ModelResult<()> {
        let txn = db.begin().await?;

        let template = templates::Entity::find_by_id(id)
            .one(&txn)
            .await?
            .ok_or(ModelError::EntityNotFound)?;

        if template.user_id != user_id {
            txn.rollback().await?;
            return Err(ModelError::Any("Unauthorized".into()));
        }

        // if files are missing... they are already deleted, so ignore errors
        let _ = fs::remove_file(format!("./data/templates/{}.docx", template.id)).await;
        let _ = fs::remove_file(format!("./data/templates/{}.dsl", template.id)).await;

        let template = template.into_active_model();

        templates::Entity::delete(template).exec(&txn).await?;

        txn.commit().await?;

        Ok(())
    }
}

impl templates::ActiveModel {
    pub async fn update_template(
        mut self,
        db: &DatabaseConnection,
        params: &CreateTemplateParams,
        id: i32,
        author_id: i32,
        docx: &[u8],
        dsl: &str,
    ) -> ModelResult<templates::Model> {
        let txn = db.begin().await?;

        compile_types(dsl).map_err(|_| ModelError::Any("Invalid DSL".into()))?;

        let template = templates::Entity::find_by_id(id)
            .one(&txn)
            .await?
            .ok_or(ModelError::EntityNotFound)?;

        if template.user_id != author_id {
            txn.rollback().await?;
            return Err(ModelError::Any("Unauthorized".into()));
        }

        let mut categories = Vec::with_capacity(params.categories.len());

        for category_id in params.categories.iter().cloned() {
            let category = categories::Entity::find_by_id(category_id)
                .one(&txn)
                .await?
                .ok_or(ModelError::EntityNotFound)?;
            categories.push(category);
        }

        let viewers = match &params.publicity {
            PublicityParams::Public => None,
            PublicityParams::Private { viewers: visible_to } => {
                let mut viewers = Vec::with_capacity(visible_to.len());

                for viewer_email in visible_to.iter().map(String::as_str) {
                    let viewer = users::Entity::find()
                        .filter(
                            query::condition()
                                .eq(users::Column::Email, viewer_email)
                                .build(),
                        )
                        .one(&txn)
                        .await?
                        .ok_or(ModelError::EntityNotFound)?;
                    viewers.push(viewer);
                }

                Some(viewers)
            },
        };

        templates_categories::Entity::delete_many()
            .filter(templates_categories::Column::TemplateId.eq(id))
            .exec(&txn)
            .await?;

        users_visible_templates::Entity::delete_many()
            .filter(users_visible_templates::Column::TemplateId.eq(id))
            .exec(&txn)
            .await?;

        fs::write(format!("./data/templates/{}.docx", template.id), docx)
            .await
            .map_err(|_| ModelError::Any("Error writing file".into()))?;

        fs::write(format!("./data/templates/{}.dsl", template.id), dsl)
            .await
            .map_err(|_| ModelError::Any("Error writing file".into()))?;

        for category in categories.iter() {
            let template_category = templates_categories::ActiveModel {
                template_id: Set(template.id),
                category_id: Set(category.id),
                ..Default::default()
            };

            templates_categories::Entity::insert(template_category)
                .exec(&txn)
                .await?;
        }

        if let Some(viewers) = viewers.as_ref() {
            for viewer in viewers.iter() {
                let viewer_template = users_visible_templates::ActiveModel {
                    user_id: Set(viewer.id),
                    template_id: Set(template.id),
                    ..Default::default()
                };

                users_visible_templates::Entity::insert(viewer_template)
                    .exec(&txn)
                    .await?;
            }
        }

        let mut template = template.into_active_model();
        template.name = Set(params.name.clone());
        template.is_public = Set(viewers.is_none());
        let template = template.update(&txn).await?;

        txn.commit().await?;

        Ok(template)
    }
}
