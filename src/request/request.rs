use super::define::Response as APIResponse;
use crate::container::api::{get_service, get_service_api};
use crate::util::json::{value_to_string};
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::HashMap;

pub struct RequestConfig {
    pub url: String,
    pub method: String,
    pub content_type: Option<String>,
    pub data_key: String,
}

impl RequestConfig {
    pub async fn do_request(
        &self,
        value: Option<Value>,
        _headers: Option<HashMap<String, String>>,
    ) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let mut builder: reqwest::RequestBuilder = client.get(&self.url);
        let mut content_type: String = String::new();
        if let Some(value) = self.content_type.as_ref() {
            content_type = value.clone();
        }
        if self.method == "GET" {
            let full_url = format!("{}?{}", self.url, build_query(&value));
            println!("{}", full_url);
            builder = client.get(&full_url);
        } else if self.method == "POST" {
            builder = client.post(&self.url);
            if let Some(params) = value {
                if content_type == "application/json" {
                    builder = builder.json(&params);
                }
            }
        }
        let response = builder.send().await?;
        Ok(response)
    }
}

fn build_query(data: &Option<Value>) -> String {
    if data.is_none() {
        return String::new();
    }
    let data = data.as_ref().unwrap();
    if !data.is_object() {
        return String::new();
    }

    let map = data.as_object().unwrap();
    let mut query = String::new();
    for (key, value) in map.iter() {
        if query.is_empty() {
            query.push_str(&format!("{}={}", key, value_to_string(value)));
        } else {
            query.push_str(&format!("&{}={}", key, value_to_string(value)));
        }
    }
    query
}

pub async fn do_request(
    service: String,
    api: String,
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
