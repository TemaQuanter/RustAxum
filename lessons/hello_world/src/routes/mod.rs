mod always_errors;
mod handling_params;
mod hello_world;
mod middleware_custom_header;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_request;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod returns_201;
mod set_middleware_custom_header;
mod returns_json;
mod validate_data;

use always_errors::always_errors;
use axum::body::Body;
use axum::http::Method;
use axum::middleware;
use axum::routing::{get, patch, post};
use axum::Extension;
use axum::Router;
use handling_params::handling_params;
use hello_world::hello_world;
use middleware_custom_header::middleware_custom_header;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_request::mirror_body_request;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use returns_201::returns_201;
use set_middleware_custom_header::set_middleware_custom_header;
use returns_json::get_json;
use validate_data::validate_data;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
} // end struct SharedData

pub fn create_routes() -> Router<(), Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data: SharedData = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/middleware_custom_header", post(middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", patch(hello_world))
        .route("/mirror_body_request", post(mirror_body_request))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/things/:id", get(path_variables))
        .route("/handling_params", post(handling_params))
        .route("/mirror_user_agent", post(mirror_user_agent))
        .route("/mirror_custom_header", post(mirror_custom_header))
        .route("/middleware_message", post(middleware_message))
        .route("/always_errors", post(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", post(get_json))
        .route("/validate_data", post(validate_data))
        .layer(cors)
        .layer(Extension(shared_data))
} // end create_routes()
