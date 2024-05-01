mod controller;

use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Extension, Router,
};
use dotenvy::dotenv;
use rusqlite::Connection;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let dbfile = "./blog.sqlite";
    let conn = Connection::open(dbfile).unwrap();
    dotenv().expect(".env not found");

    let app = Router::new()
        .route("/", get(handler))
        .route(
            "/posts",
            get(controller::posts::get_posts).post(controller::posts::create_posts),
        )
        .route("/s3api", get(controller::s3api::get_uploader))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
        .layer(Extension(Arc::new(Mutex::new(conn))));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    (StatusCode::OK, Json(json!("OK")))
}
