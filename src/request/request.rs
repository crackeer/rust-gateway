use crate::container::api::{get_service, get_service_api};
use crate::util::json::{get_json_value, get_json_value_string};
use crate::util::request::build_query;
use serde_json::Value;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
pub struct APIConfig {
    pub url: String,
    pub method: String,
    pub content_type: Option<String>,
    pub data_key: String,
    pub message_key: String,
    pub code_key: String,
    pub success_code: Vec<String>,
}

pub struct RequestWrap {
    pub api_config: APIConfig,
    pub name: String,
    pub params: Value,
    pub headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseWrap {
    pub name: String,
    pub error: Option<String>,
    pub error_code: i64,
    pub business_code: String,
    pub data: Option<Value>,
    pub message: String,
    pub cost: i64,
}

impl ResponseWrap {
    fn new_error_response(name: String, error_message: String, error_code: i64) -> ResponseWrap {
        return ResponseWrap {
            error: Some(error_message),
            error_code: error_code,
            name: name,
            business_code: String::from(""),
            data: None,
            message: String::from(""),
            cost: 0,
        };
    }
}

pub async fn do_simple_request(wrapper: &RequestWrap) -> ResponseWrap {
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
        if content_type == "application/json" {
            builder = builder.json(&wrapper.params);
        }
    }
    let result = builder.send().await;
    if result.is_err() {
        return ResponseWrap::new_error_response(
            wrapper.name.clone(),
            result.err().unwrap().to_string(),
            -100,
        );
    }
    let response = result.unwrap();
    if response.status() != 200 {
        return ResponseWrap::new_error_response(
            wrapper.name.clone(),
            format!("http status {}", response.status()),
            -200,
        );
    }

    let json_value: Value = response.json().await.unwrap();
    let data_value = get_json_value(&json_value, wrapper.api_config.data_key.as_str());
    let code_value = get_json_value_string(&json_value, wrapper.api_config.code_key.as_str());
    let message_value = get_json_value_string(&json_value, wrapper.api_config.message_key.as_str());
    return ResponseWrap {
        error: None,
        error_code: 0,
        name: wrapper.name.clone(),
        data: data_value,
        business_code: code_value,
        message: message_value.clone(),
        cost: 0,
    };
}

pub async fn do_request(
    service: String,
    api: String,
    params: &Value,
    headers: &HashMap<String, String>,
    name: String,
) -> ResponseWrap {
    let service_config = get_service(&service);
    if service_config.is_none() {
        return ResponseWrap::new_error_response(
            name.clone(),
            String::from("No service specified"),
            -300,
        );
    }
    let service_config = service_config.unwrap();

    let api_config = get_service_api(&service, &api);
    if api_config.is_none() {
        return ResponseWrap::new_error_response(
            name.clone(),
            String::from("No service api specified"),
            -300,
        );
    }
    let api_config = api_config.unwrap();

    let full_url_path = format!("{}/{}", service_config.host, api_config.path);

    let wrapper = &RequestWrap {
        api_config: APIConfig {
            url: full_url_path,
            method: api_config.method.clone(),
            content_type: api_config.content_type.clone(),
            data_key: service_config.data_key.clone(),
            message_key: service_config.message_key.clone(),
            code_key: service_config.code_key.clone(),
            success_code: vec![service_config.success_code.clone()],
        },
        name: name,
        params: params.clone(),
        headers: headers.clone(),
    };

    return do_simple_request(wrapper).await;
}
