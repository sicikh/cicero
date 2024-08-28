use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(UsersVisibleTemplates::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-users_visible_templates-refs-pk")
                            .table(UsersVisibleTemplates::Table)
                            .col(UsersVisibleTemplates::TemplateId)
                            .col(UsersVisibleTemplates::UserId)
                            ,
                    )
                    .col(integer(UsersVisibleTemplates::TemplateId))
                    .col(integer(UsersVisibleTemplates::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_visible_templates-templates")
                            .from(UsersVisibleTemplates::Table, UsersVisibleTemplates::TemplateId)
                            .to(Templates::Table, Templates::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_visible_templates-users")
                            .from(UsersVisibleTemplates::Table, UsersVisibleTemplates::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UsersVisibleTemplates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UsersVisibleTemplates {
    Table,
    TemplateId,
    UserId,
    
}


#[derive(DeriveIden)]
enum Templates {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
