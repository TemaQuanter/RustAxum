use std::env;

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    now += Duration::seconds(30);
    let exp = now.timestamp() as usize;
    let claim = Claims { exp, iat };
    let secret = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
    encode(&Header::default(), &claim, &secret).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<(), AppError> {
    let secret = env::var("JWT_SECRET").unwrap();
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::default()).map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
            StatusCode::UNAUTHORIZED,
            "Your session has expired, please log in again",
        ),
        _ => AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        ),
    })?;
    Ok(())
}
