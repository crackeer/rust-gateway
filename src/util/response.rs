use axum::response::{IntoResponse, Response};
use bytes::{BufMut, BytesMut};
use http::{
    header::{self, HeaderValue},
    StatusCode,
};
use mime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const SUCCESS_MESSAGE: &'static str = "success";
const SUCCESS_CODE: u32 = 0;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardResponse {
    code: u32,
    data: Option<Value>,
    cost: u32,
    message: String,
}

impl IntoResponse for StandardResponse {
    fn into_response(self) -> Response {
        let mut buf = BytesMut::new().writer();
        match serde_json::to_writer(&mut buf, &self) {
            Ok(()) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                buf.into_inner().freeze(),
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

#[allow(dead_code)]
impl StandardResponse {
    pub fn new() -> StandardResponse {
        StandardResponse {
            code: SUCCESS_CODE,
            data: None,
            cost: 0,
            message: SUCCESS_MESSAGE.to_string(),
        }
    }
    pub fn success(data: Value) -> StandardResponse {
        StandardResponse {
            code: SUCCESS_CODE,
            data: Some(data),
            cost: 0,
            message: SUCCESS_MESSAGE.to_string(),
        }
    }
    pub fn failure(code: u32, message: String) -> StandardResponse {
        StandardResponse {
            code: code,
            data: None,
            cost: 0,
            message: message.clone(),
        }
    }
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data.clone());
        return self;
    }
    pub fn with_cost(mut self, cost: u32) -> Self {
        self.cost = cost;
        return self;
    }
    pub fn with_message(mut self, message: String) -> Self {
        self.message = message.clone();
        return self;
    }
    pub fn with_code(mut self, code: u32) -> Self {
        self.code = code;
        return self;
    }
}
