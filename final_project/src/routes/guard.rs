use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::is_valid},
};

pub async fn guard<T>(
    State(database): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(&database)
        .await
        .map_err(|_err| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    let Some(user) = user else {return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized, please log in or create an account"))};

    is_valid(&token)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
