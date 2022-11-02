use crate::util::request as util_request;

use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::{FromRequest, Path, Query, RequestParts},
    http::{header::CONTENT_TYPE, response, StatusCode, Uri},
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
pub struct MeshParams {
    path: String,
    params: Option<Value>,
    header: Option<HashMap<String, String>>,
}

#[async_trait]
impl<B> FromRequest<B> for MeshParams
where
    B: Send,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let header: HashMap<String, String> = util_request::extract_header(req);
        let data: Value = util_request::extract_parameter_all(req).await;

        return Ok(MeshParams {
            params: Some(data),
            header: Some(header),
            path: req.uri().to_string(),
        });
    }
}

pub async fn mesh(params: MeshParams) -> impl IntoResponse {
    axum::Json(params)
    /*
    let result = do_request(params.service, params.api, params.params, params.header).await;
    if let Ok(response) = result {
        return axum::Json(response);
    }
    axum::Json(APIResponse {
        data: None,
        code: 10001,
        cost: 0,
        message: result.err().unwrap(),
    })
    */
}