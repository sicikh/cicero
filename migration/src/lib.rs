#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20240827_164857_templates;
mod m20240827_165252_categories;
mod m20240827_165502_templates_categories;

mod m20240827_171617_users_visible_templates;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20240827_164857_templates::Migration),
            Box::new(m20240827_165252_categories::Migration),
            Box::new(m20240827_165502_templates_categories::Migration),
            Box::new(m20240827_171617_users_visible_templates::Migration),
        ]
    }
}