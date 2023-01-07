use crate::container::api::get_router_config;
use crate::request::mesh::do_mesh_request;
use crate::util::request as util_request;
use crate::util::json::extract_value;
use crate::util::response::{StandardResponse};
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
    BoxError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct MeshParams {
    path: String,
    params: Value,
    header: HashMap<String, String>,
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
            params: data.clone(),
            header: header,
            path: req.uri().path().to_string(),
        });
    }
}

pub async fn mesh(params: MeshParams) -> impl IntoResponse {
    let router_config = get_router_config(&params.path);
    let mut response : Value = json!({});
    if let Some(router) = router_config {
        if let Ok(result) = do_mesh_request(router.config, &params.params, &params.header).await {
            response = result
        }
        if let Some(dest_response) = router.response {
            if let Some(value) = extract_value(&response, &dest_response) {
                response = value
            }
        }
    }
    StandardResponse::success(response)
    //axum::Json(response)
}
