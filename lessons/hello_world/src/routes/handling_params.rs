use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueryData {
    message: String,
    id: usize,
} // end struct QueryData

pub async fn handling_params(Query(query): Query<QueryData>) -> Json<QueryData> {
    Json(QueryData {
        message: query.message,
        id: query.id,
    })
} // end handling_params()
