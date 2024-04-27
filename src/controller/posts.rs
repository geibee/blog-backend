use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
pub struct CreatePostsRequest {
    pub caption: String,
    pub image_url: String,
}

#[derive(Serialize)]
struct Post {
    id: String,
    caption: String,
    image_url: String,
}

pub async fn get_posts(Extension(conn): Extension<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, caption, image_url from posts")
        .unwrap();

    let posts = stmt
        .query_map([], |row| {
            Ok(Post {
                id: row.get(0)?,
                caption: row.get(1)?,
                image_url: row.get(2)?,
            })
        })
        .unwrap()
        .map(|row| row.unwrap())
        .collect::<Vec<Post>>();

    (StatusCode::OK, Json(json!(posts)))
}

pub async fn create_posts(
    Extension(conn): Extension<Arc<Mutex<Connection>>>,
    Json(payload): Json<CreatePostsRequest>,
) -> impl IntoResponse {
    let conn = conn.lock().unwrap();
    conn.execute(
        "INSERT INTO posts(caption, image_url) values (?1, ?2)",
        (&payload.caption, &payload.image_url),
    )
    .unwrap();

    StatusCode::CREATED
}
