use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(TemplatesCategories::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-templates_categories-refs-pk")
                            .table(TemplatesCategories::Table)
                            .col(TemplatesCategories::TemplateId)
                            .col(TemplatesCategories::CategoryId)
                            ,
                    )
                    .col(integer(TemplatesCategories::TemplateId))
                    .col(integer(TemplatesCategories::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-templates_categories-templates")
                            .from(TemplatesCategories::Table, TemplatesCategories::TemplateId)
                            .to(Templates::Table, Templates::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-templates_categories-categories")
                            .from(TemplatesCategories::Table, TemplatesCategories::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TemplatesCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TemplatesCategories {
    Table,
    TemplateId,
    CategoryId,
    
}


#[derive(DeriveIden)]
enum Templates {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Categories {
    Table,
    Id,
}
