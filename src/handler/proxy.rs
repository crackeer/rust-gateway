use crate::request::request::do_request;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, Path, Query, RequestParts},
    http::{header::CONTENT_TYPE, StatusCode, Uri},
    response::{IntoResponse, Response},
    BoxError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{hash_map::RandomState, HashMap},
    io::Read,
};

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
        let mut data: Value = json!({});
        let query: Query<HashMap<String, String, RandomState>> =
            Query::from_request(req).await.unwrap();

        let object = data.as_object_mut().unwrap();
        for (key, value) in query.iter() {
            object.insert(String::from(key), Value::String(value.clone()));
        }

        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if req.method().to_string().eq("POST") {
            if let Some(content_type) = content_type {
                if content_type == "application/json" {
                    let bytes = Bytes::from_request(req).await.unwrap();
                    let post_data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                    if let Value::Object(post_map) = post_data {
                        for (key, value) in post_map.iter() {
                            object.insert(String::from(key), value.clone());
                        }
                    }
                }
            }
        }

        return Ok(Params {
            params: Some(data),
            api: tmp_params.api,
            service: tmp_params.service,
            header: Some(header),
        });
    }
}

pub async fn relay(params: Params) -> impl IntoResponse {
    let response = do_request(params.service, params.api, params.params, params.header).await;
    axum::Json(response)
}
