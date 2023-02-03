mod youtube;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    const PORT: u32 = 3000u32;

    let app = Router::new()
        .route("/youtube/live", get(youtube::live));

    println!("Listening on http://127.0.0.1:{}", PORT);

    axum::Server::bind(&format!("0.0.0.0:{}", PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
