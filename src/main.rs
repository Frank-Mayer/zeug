mod youtube;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build application with a single route
    let app = Router::new()
        .route("/youtube/live", get(youtube::live));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
