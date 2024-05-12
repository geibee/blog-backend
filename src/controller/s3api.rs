use axum::{
    extract::Query,
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

#[derive(Serialize, Deserialize)]
pub struct S3Object {
    pub presigned_url: String,
}

#[derive(Deserialize)]
pub struct GetParams {
    original_url: String,
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

async fn get_object(
    client: &Client,
    bucket: &str,
    object: &str,
    expires_in: u64,
) -> Result<String, Box<dyn Error>> {
    let expires_in = Duration::from_secs(expires_in);
    let presigned_request = client
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;
    Ok(presigned_request.uri().to_string())
}

pub async fn get_viewer(Query(params): Query<GetParams>) -> impl IntoResponse {
    let shared_config = aws_config::from_env()
        .region(Region::new("ap-northeast-1"))
        .load()
        .await;
    let client = Client::new(&shared_config);
    let bucket = env::var("IMAGE_BUCKET").unwrap();

    let key: &str = &params
        .original_url
        .split("com/")
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Index  out of bounds");

    let object_url = get_object(&client, &bucket, key, 900).await.unwrap();

    (
        StatusCode::OK,
        Json(json!(S3Object {
            presigned_url: object_url,
        })),
    )
}
