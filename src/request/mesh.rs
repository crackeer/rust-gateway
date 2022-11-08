use super::define::Response as APIResponse;
use crate::container::api::{get_service, get_service_api};
use crate::util::json::{value_to_string};
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::HashMap;
use crate::request::define::{RouterRequestCell};

pub async fn do_mesh_request(
    cells: Vec<Vec<RouterRequestCell>>,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<APIResponse, String> {
    let service_config = get_service(&service);
    if service_config.is_none() {
        return Err(String::from("No service specified"));
    }
    let service_config = service_config.unwrap();

    let api_config = get_service_api(&service, &api);
    if api_config.is_none() {
        return Err(String::from("No service api specified"));
    }
    let api_config = api_config.unwrap();

    let full_url_path = format!("{}/{}", service_config.host, api_config.path);

    let request_config = &RequestConfig {
        url: full_url_path,
        method: api_config.method.clone(),
        content_type: api_config.content_type.clone(),
        data_key: service_config.data_key.clone(),
    };

    let response = request_config.do_request(params, headers).await;
    if let Ok(response) = response {
        let ddd: Value = response.json().await.unwrap();
        let data_value = ddd.pointer(service_config.data_key.as_str()).unwrap().to_owned();
        let code_value = ddd.pointer(service_config.code_key.as_str()).unwrap().to_owned();
        let message_value = ddd.pointer(service_config.message_key.as_str()).unwrap().to_owned();
        return Ok(APIResponse {
            data: Some(data_value),
            code: code_value.as_u64().unwrap(),
            cost: 0,
            message: message_value.as_str().unwrap().to_string(),
        });
    }
    Err(String::from(response.err().unwrap().to_string()))
}
