use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use chrono::Local;

use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{config::Region, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::time::Duration;
use std::{env, error::Error};

#[derive(Deserialize)]
pub struct MediaFile {
    pub name: String,
}

#[derive(Serialize)]
pub struct S3Object {
    pub presigned_url: String,
}

async fn put_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    println!("Object URI: {}", presigned_request.uri());

    Ok(presigned_request.uri().to_string())
}

pub async fn generate_uploader(Json(payload): Json<MediaFile>) -> impl IntoResponse {
    let shared_config = aws_config::from_env()
        .region(Region::new("ap-northeast-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let bucket = env::var("IMAGE_BUCKET").unwrap();

    let local_datetime = Local::now().format("%Y%m%d%H%M%S").to_string();
    let key = format!("{}_{}", local_datetime, &payload.name);
    let object_url = put_object(&client, &bucket, &key, 900).await.unwrap();

    (
        StatusCode::OK,
        Json(json!(S3Object {
            presigned_url: object_url,
        })),
    )
}
