use std::ptr::read_unaligned;

use axum::Extension;
use axum::Json;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::Set;
use serde::Deserialize;

use crate::database::tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) {
    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();

    dbg!(result);
}
