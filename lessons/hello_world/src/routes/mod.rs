mod hello_world;

use axum::Router;
use axum::routing::patch;
use axum::body::Body;
use hello_world::hello_world;

pub fn create_routes() -> Router<(), Body> {
    Router::new().route("/", patch(hello_world))
} // end create_routes()