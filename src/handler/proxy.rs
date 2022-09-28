use axum::{
    async_trait,
    http::{StatusCode,header::CONTENT_TYPE, Request},
    response::{IntoResponse, Response},
    Json, Router,
    extract::{Query, FromRequest, RequestParts},
    http::request::Parts,
    Form,
    body::{Bytes, HttpBody},
    BoxError,
};
use serde_json::{Value};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::service_api::{api::get_md_list};
use reqwest;

#[derive(Debug, Deserialize, Serialize)]

pub async fn relay(
    params: Params
) -> impl IntoResponse {
    axum::Json(params)
}


pub struct Params {
    service: String,
    api: String,
    params : Option<Value>,
}

#[async_trait]
impl<B> FromRequest<B> for Params where
    B: Send,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        let method = req.method();
        println!("{}{}", method.to_string(), content_type.is_some());
        if method.to_string().eq("POST") {
            if let Some(content_type) = content_type {
                let bytes  = Bytes::from_request(req).await.unwrap();
               
                let data : Value = serde_json::from_slice(&bytes).unwrap();
                return Ok(Params { params: Some(data), api : String::from("ss"), service: String::from("") });
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}