use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::Deserialize;

use crate::database::tasks::{self, Entity as Tasks};

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };

        let now = chrono::Utc::now();

        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        // Approach 1
        //
        // let mut task = if let Some(task) = Tasks::find_by_id(task_id)
        //     .one(&database)
        //     .await
        //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        // {
        //     task.into_active_model()
        // } else {
        //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
        // };
        //
        // Tasks::delete(task)
        //     .exec(&database)
        //     .await
        //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Approach 2
        //
        // Tasks::delete_by_id(task_id)
        //     .exec(&database)
        //     .await
        //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Approach 3
        //
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
