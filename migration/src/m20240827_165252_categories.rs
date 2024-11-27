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
                table_auto_tz(Categories::Table)
                    .col(pk_auto(Categories::Id))
                    .col(string_uniq(Categories::Name))
                    .col(integer(Categories::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-categories-users")
                            .from(Categories::Table, Categories::UserId)
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
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
    Name,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
