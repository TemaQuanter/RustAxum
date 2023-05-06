use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct JsonData {
    message: String,
    account: i64,
    username: String
} // end struct JsonData

pub async fn get_json() -> Json<JsonData> {
    Json(JsonData {
        message: "You asked for a JSON, here it is :)".to_string(),
        account: 3_000_937,
        username: "Artem Mikheev".to_string()
    }) // end Json
} // end get_json()