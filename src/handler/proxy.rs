use axum::{
    async_trait,
    http::{StatusCode,header::CONTENT_TYPE, Request},
    response::IntoResponse,
    Json, Router,
    extract::{Path, Query, FromRequest},
    http::request::Parts,
};
use serde_json::{Value};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::service_api::{api::get_md_list};
use reqwest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    service: String,
    api: String,
    params : Option<Value>,
}

pub async fn relay(
    Path(params): Path<Params>
) -> impl IntoResponse {
    axum::Json(params)
}

#[async_trait]
impl<S, B, T, U> FromRequest<S, B> for Params<T, U> where
    B: Send + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<(), B>,
    Form<U>: FromRequest<(), B>,
    T: 'static,
    U: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self::Json(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self::Form(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}
