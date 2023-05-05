use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
} // end struct Message

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    from: String,
    to: String,
    message: String,
}

pub async fn mirror_body_json(Json(body): Json<Message>) -> Json<MessageResponse> {
    Json(MessageResponse {
        from: "Server".to_string(),
        to: "ThunderClient".to_string(),
        message: "Got your message, buddy!".to_string(),
    })
} // end fn mirror_body_json()
