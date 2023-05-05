use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
} // end middleware_custom_header()
