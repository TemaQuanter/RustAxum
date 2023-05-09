mod create_task;
mod custom_json_extractor;
mod delete_task;
mod get_tasks;
mod partial_update_tasks;
mod test;
mod update_tasks;

use axum::routing::{delete, get, patch, post, put};
use axum::Router;
use axum::{body::Body, Extension};

use sea_orm::DatabaseConnection;

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use delete_task::delete_task;
use get_tasks::get_all_tasks;
use get_tasks::get_one_task;
use partial_update_tasks::partial_update;
use test::test;
use update_tasks::atomic_update;

pub async fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/test", post(test))
        .route("/tasks", post(create_task))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks/:task_id", post(get_one_task))
        .route("/tasks/:task_id", put(atomic_update))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_task))
        .layer(Extension(database))
} // end create_routes()
