mod controller;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler)).route(
        "/posts",
        get(controller::posts::get_posts).post(controller::posts::create_posts),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    (StatusCode::OK, Json(json!("OK")))
}
