use axum::debug_handler;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use futures;
use reqwest;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};

use super::s3api::S3Object;

#[derive(Deserialize)]
pub struct CreatePostsRequest {
    pub caption: String,
    pub image_url: String,
}

#[derive(Serialize, Clone)]
struct Post {
    id: u64,
    caption: String,
    image_url: String,
    created_at: String,
    updated_at: String,
}

#[debug_handler]
pub async fn get_posts(Extension(conn): Extension<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let posts = {
        let conn = conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, caption, image_url, created_at, updated_at from posts")
            .unwrap();

        let posts = stmt
            .query_map([], |row| {
                Ok(Post {
                    id: row.get(0)?,
                    caption: row.get(1)?,
                    image_url: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .unwrap()
            .map(|row| row.unwrap())
            .collect::<Vec<Post>>();
        posts
    };
    let arc_posts = Arc::new(tokio::sync::Mutex::new(posts));
    let updated_posts: Vec<Post> = update_url(arc_posts).await;

    (StatusCode::OK, Json(json!(updated_posts)))
}

async fn update_url(posts: Arc<tokio::sync::Mutex<Vec<Post>>>) -> Vec<Post> {
    let posts = posts.lock();
    let new_posts = futures::future::join_all(posts.await.iter_mut().map(|post| {
        let client = reqwest::Client::new();
        let url = post.image_url.clone();
        let new_url = format!("http://localhost:3000/s3api?original_url={}", url);
        let post_clone = post.clone();
        async move {
            match client.get(&new_url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<S3Object>().await {
                            Ok(data) => Post {
                                image_url: data.presigned_url,
                                ..post_clone
                            },
                            Err(_) => post_clone,
                        }
                    } else {
                        post_clone
                    }
                }
                Err(_) => post_clone,
            }
        }
    }))
    .await;
    new_posts
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
