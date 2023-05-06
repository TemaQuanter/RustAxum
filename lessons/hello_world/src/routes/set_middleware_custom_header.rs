use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extension = request.extensions_mut();
    extension.insert(HeaderMessage(message.to_owned()));

    Ok(next.run(request).await)
} // end set_middleware_custom_header()
