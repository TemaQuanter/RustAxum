use axum::http::{HeaderMap, HeaderValue};

pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message: &HeaderValue = headers.get("my_header").unwrap();
    let message = message.to_str().unwrap().to_owned();
    message
} // end mirror_custom_header()
