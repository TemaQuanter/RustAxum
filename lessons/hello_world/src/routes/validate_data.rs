use axum::Json;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct UserData {
    login: Option<String>,
    password: Option<String>
}

pub async fn validate_data(Json(user): Json<UserData>) {
    dbg!(user);
} // end validate_data()