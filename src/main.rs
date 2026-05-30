mod handlers;
mod migrations;

use cot::auth::db::DatabaseUserApp;
use cot::cli::CliMetadata;
use cot::db::migrations::SyncDynMigration;
use cot::middleware::{AuthMiddleware, LiveReloadMiddleware, SessionMiddleware};
use cot::openapi::swagger_ui::SwaggerUi;
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandler, RootHandlerBuilder};
use cot::router::method::openapi::{api_get, api_post};
use cot::router::{Route, Router};
use cot::session::db::SessionApp;
use cot::static_files::{StaticFile, StaticFilesMiddleware};
use cot::{App, AppBuilder, Project, static_files};

struct RustyHatchetTasksApp;

impl App for RustyHatchetTasksApp {
    fn name(&self) -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }

    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler_and_name("/", handlers::index, "index"),
            Route::with_handler_and_name("/tasks", handlers::tasks, "tasks"),
            Route::with_api_handler_and_name(
                "/api/tasks",
                api_post(handlers::create_task),
                "create_task",
            ),
            Route::with_api_handler_and_name(
                "/api/tasks/{task_id}",
                api_get(handlers::get_task),
                "get_task",
            ),
        ])
    }

    fn static_files(&self) -> Vec<StaticFile> {
        static_files!("css/main.css", "js/main.js")
    }
}

struct RustyHatchetTasksProject;

impl Project for RustyHatchetTasksProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn register_apps(&self, apps: &mut AppBuilder, _context: &RegisterAppsContext) {
        apps.register_with_views(SwaggerUi::new(), "/swagger");
        apps.register_with_views(RustyHatchetTasksApp, "");
        apps.register(DatabaseUserApp::new());
        apps.register(SessionApp::new());
    }

    fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
        handler
            .middleware(StaticFilesMiddleware::from_context(context))
            .middleware(AuthMiddleware::new())
            .middleware(SessionMiddleware::from_context(context))
            .middleware(LiveReloadMiddleware::from_context(context))
            .build()
    }
}

#[cot::main]
fn main() -> impl Project {
    RustyHatchetTasksProject
}
