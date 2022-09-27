use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{Path, Query},
    http::request::Parts,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::service_api::{api::get_md_list};
use reqwest;


pub async fn relay(
    Path(service_api): Path<String>
) -> impl IntoResponse {
    println!("{}", service_api);
    (StatusCode::OK, service_api)
}
