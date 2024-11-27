use std::path::Path;

use async_trait::async_trait;
use loco_rs::app::{AppContext, Hooks, Initializer};
use loco_rs::bgworker::Queue;
use loco_rs::boot::{create_app, BootResult, StartMode};
use loco_rs::controller::AppRoutes;
use loco_rs::db::{self, truncate_table};
use loco_rs::environment::Environment;
use loco_rs::task::Tasks;
use loco_rs::Result;
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::models::_entities::users;
use crate::models::{categories, templates, templates_categories};
use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::templates::routes())
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }
    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
        // tasks-inject (do not remove)
    }
    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<templates::ActiveModel>(db, &base.join("templates.yaml").display().to_string())
            .await?;
        db::seed::<categories::ActiveModel>(
            db,
            &base.join("categories.yaml").display().to_string(),
        )
        .await?;
        // should be error, because loco-rs updates auto-increment via `id` column,
        // which is absent in the table
        let _ = db::seed::<templates_categories::ActiveModel>(
            db,
            &base.join("templates_categories.yaml").display().to_string(),
        )
        .await;
        Ok(())
    }
}
