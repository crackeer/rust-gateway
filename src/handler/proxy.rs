use crate::service_api::api::get_md_list;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, Query, RequestParts, Path},
    http::request::Parts,
    http::{header::CONTENT_TYPE, Request, StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    BoxError, Form, Json, Router,
};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, io::Read};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    service: String,
    api: String,
    params: Option<Value>,
    header : Option<HashMap<String, String>>,
}

#[async_trait]
impl<B> FromRequest<B> for Params
where
    B: Send,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {

        let result = Path::from_request(req).await;
        if result.is_err() {
            println!("{}", result.as_ref().err().unwrap());
        }
        let path_params : Path<Params> = result.unwrap();
        let Path(tmp_params) = path_params;
        let mut header : HashMap<String, String> = HashMap::new();
        for (key, value) in req.headers().iter() {
            let mut tmp_str = String::from("");
            if value.clone().as_bytes().read_to_string(&mut tmp_str).is_ok() {
                header.insert(key.to_string(), tmp_str);
            }
        }
       
        
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        let method = req.method();

        if method.to_string().eq("POST") {
            if let Some(content_type) = content_type {
                let bytes = Bytes::from_request(req).await.unwrap();

                let data: Value = serde_json::from_slice(&bytes).unwrap();
                return Ok(Params {
                    params: Some(data),
                    api: tmp_params.api,
                    service: tmp_params.service,
                    header: Some(header),
                });
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}

pub async fn relay(params: Params) -> impl IntoResponse {
    axum::Json(params)
}
