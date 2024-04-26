use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreatePostsRequest {
    pub post_id: String,
}

pub async fn get_posts() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(
            json!({"post_id": "1", "caption": "うちのいぬ", "image_url": "https://via.placeholder.com/500x200"}),
        ),
    )
}

pub async fn create_posts(Json(_payload): Json<CreatePostsRequest>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({"caption": "うちのいぬ", "image_url": "https://via.placeholder.com/500x200"})),
    )
}
