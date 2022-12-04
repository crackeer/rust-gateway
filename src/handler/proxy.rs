use crate::request::request::do_request;
use crate::util::request as util_request;

use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, Path, RequestParts},
    response::{IntoResponse, Response},
    BoxError,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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

        let header: HashMap<String, String> = util_request::extract_header(req);
        let data: Value = util_request::extract_parameter_all(req).await;
        let path_params: Path<Params> = result.unwrap();
        let Path(tmp_params) = path_params;

        return Ok(Params {
            params: Some(data),
            api: tmp_params.api,
            service: tmp_params.service,
            header: Some(header),
        });
    }
}

pub async fn relay(params: Params) -> impl IntoResponse {
    let result = do_request(
        params.service,
        params.api,
        params.params,
        params.header,
        String::from("simple"),
    )
    .await;
    return axum::Json(result);
}
