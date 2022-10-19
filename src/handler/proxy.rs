use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, Path, RequestParts},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    BoxError, 
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap,  io::Read};
use crate::request::request::{do_request, do_request1};
#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    service: String,
    api: String,
    params: Option<Value>,
    header: Option<HashMap<String, String>>,
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
        let path_params: Path<Params> = result.unwrap();
        let Path(tmp_params) = path_params;
        let mut header: HashMap<String, String> = HashMap::new();
        for (key, value) in req.headers().iter() {
            let mut tmp_str = String::from("");
            if value
                .clone()
                .as_bytes()
                .read_to_string(&mut tmp_str)
                .is_ok()
            {
                header.insert(key.to_string(), tmp_str);
            }
        }

        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        let method = req.method();

        if method.to_string().eq("POST") {
            if let Some(_content_type) = content_type {
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
    println!("{}{}", params.api, params.service);
    let response = do_request(params.service, params.api, params.params, params.header).await;
    axum::Json(response)
}
