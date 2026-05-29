mod handlers;
mod migrations;

use cot::auth::db::DatabaseUserApp;
use cot::cli::CliMetadata;
use cot::db::migrations::SyncDynMigration;
use cot::middleware::{AuthMiddleware, LiveReloadMiddleware, SessionMiddleware};
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandler, RootHandlerBuilder};
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
            Route::with_handler_and_name("/run_task", handlers::run_task, "run_task"),
            Route::with_handler_and_name(
                "/get_task_status/{task_id}",
                handlers::get_task_status,
                "get_task_status",
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
