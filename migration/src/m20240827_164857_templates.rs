use loco_rs::schema::table_auto_tz;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Templates::Table)
                    .col(pk_auto(Templates::Id))
                    .col(string(Templates::Name))
                    .col(string(Templates::Description))
                    .col(integer(Templates::UserId))
                    .col(boolean(Templates::IsPublic))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-templates-users")
                            .from(Templates::Table, Templates::UserId)
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
            .drop_table(Table::drop().table(Templates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Templates {
    Table,
    Id,
    Name,
    UserId,
    IsPublic,
    Description,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
