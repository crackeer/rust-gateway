use axum::{ http::StatusCode, response::IntoResponse, Json};
use crate::service::work::{Work, download_work_to};
use std::path::Path;

pub async fn download_work(Json(payload): Json<Work>) -> impl IntoResponse {
    download_work_to(&payload, Path::new("./work")).await;
    (StatusCode::CREATED, Json(payload))
}