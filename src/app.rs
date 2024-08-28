use std::path::Path;

use async_trait::async_trait;
use loco_rs::app::{AppContext, Hooks, Initializer};
use loco_rs::boot::{create_app, BootResult, StartMode};
use loco_rs::controller::AppRoutes;
use loco_rs::db::{self, truncate_table};
use loco_rs::environment::Environment;
use loco_rs::storage::{self, Storage};
use loco_rs::task::Tasks;
use loco_rs::worker::{AppWorker, Processor};
use loco_rs::Result;
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::models::_entities::users;
use crate::models::{categories, templates, templates_categories};
use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::templates::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {}

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<categories::ActiveModel>(
            db,
            &base.join("categories.yaml").display().to_string(),
        )
        .await?;
        db::seed::<templates::ActiveModel>(db, &base.join("templates.yaml").display().to_string())
            .await?;
        db::seed::<templates_categories::ActiveModel>(
            db,
            &base.join("templates_categories.yaml").display().to_string(),
        )
        .await?;
        Ok(())
    }
}
