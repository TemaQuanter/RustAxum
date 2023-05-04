mod routes;

pub async fn run() {
    let app = routes::create_routes();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
} // end run()
