mod test;
mod create_task;

use axum::{body::Body, Extension};
use axum::routing::post;
use axum::Router;

use sea_orm::DatabaseConnection;

use test::test;
use create_task::create_task;

pub async fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/test", post(test))
        .route("/tasks", post(create_task))
        .layer(Extension(database))
} // end create_routes()
