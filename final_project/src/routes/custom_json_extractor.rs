use axum::async_trait;
use axum::body::HttpBody;
use axum::extract::FromRequest;
use axum::http::Request;
use axum::http::StatusCode;
use axum::BoxError;
use axum::Json;
use axum::RequestExt;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct RequestUser {
    #[validate(email(message = "Must be a valid email"))]
    username: String,
    #[validate(length(min = 8, message = "Must be at least 8 characters"))]
    password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for RequestUser
where
    // these bounds are required by `async_trait`
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
