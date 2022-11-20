use super::define::Response as APIResponse;
use crate::container::api::{get_service, get_service_api};
use crate::util::json::{get_json_value, get_json_value_string};
use crate::util::request::build_query;
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::HashMap;
pub struct APIConfig {
    pub url: String,
    pub method: String,
    pub content_type: Option<String>,
    pub data_key: String,
}

pub struct RequestWrapper {
    pub api_config: APIConfig,
    pub params: Option<Value>,
    pub headers: Option<HashMap<String, String>>,
}

pub async fn do_simple_request(wrapper: &RequestWrapper) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let mut builder: reqwest::RequestBuilder = client.get(&wrapper.api_config.url);
    let mut content_type: String = String::new();
    if let Some(value) = wrapper.api_config.content_type.as_ref() {
        content_type = value.clone();
    }
    if wrapper.api_config.method == "GET" {
        let full_url = format!(
            "{}?{}",
            wrapper.api_config.url,
            build_query(&wrapper.params)
        );
        println!("{}", full_url);
        builder = client.get(&full_url);
    } else if wrapper.api_config.method == "POST" {
        builder = client.post(&wrapper.api_config.url);
        if let Some(params) = &wrapper.params {
            if content_type == "application/json" {
                builder = builder.json(&params);
            }
        }
    }
    let response = builder.send().await?;
    Ok(response)
}

pub async fn do_request(
    service: String,
    api: String,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
    name: String,
) -> Result<APIResponse, String> {
    let service_config = get_service(&service);
    if service_config.is_none() {
        return Err(String::from("No service specified"));
    }
    let service_config = service_config.unwrap();

    let api_config = get_service_api(&service, &api);
    if api_config.is_none() {
        /*
         return Ok(APIResponse {
            name : name.clone(),
            data: None,
            code: 0,
            cost: 0,
            business_code: String::from(""),
            message: message_value.unwrap().to_string(),
        });
        */
        return Err(String::from("No service api specified"));
    }
    let api_config = api_config.unwrap();

    let full_url_path = format!("{}/{}", service_config.host, api_config.path);

    let wrapper = &RequestWrapper {
        api_config: APIConfig {
            url: full_url_path,
            method: api_config.method.clone(),
            content_type: api_config.content_type.clone(),
            data_key: service_config.data_key.clone(),
        },
        params: params,
        headers: headers,
    };

    let response = do_simple_request(wrapper).await;
    if let Ok(response) = response {
        if response.status() != 200 {}
        let json_value: Value = response.json().await.unwrap();
        let data_value = get_json_value(&json_value, service_config.data_key.as_str());
        let code_value = get_json_value_string(&json_value, service_config.data_key.as_str());
        let message_value = get_json_value(&json_value, service_config.message_key.as_str());
        return Ok(APIResponse {
            name: name.clone(),
            data: data_value,
            code: 0,
            cost: 0,
            business_code: code_value,
            message: message_value.unwrap().to_string(),
        });
    }
    Err(String::from(response.err().unwrap().to_string()))
}
