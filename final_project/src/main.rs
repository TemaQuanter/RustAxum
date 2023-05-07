use dotenv::dotenv;
use final_project::run;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = env::var("DATABASE_URL").unwrap();
    run(&database_uri).await;
}
