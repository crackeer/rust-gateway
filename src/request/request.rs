use crate::container::timer::{API_MAP, SERVICE_MAP};
use reqwest::{Error, Response};
use serde_json::Value;
use std::collections::HashMap;

pub struct RequestConfig {
    pub url: String,
    pub method: String,
    pub content_type: Option<String>,
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
        if self.method == "POST" {
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

pub async fn do_request(
    service: String,
    api: String,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<Value, String> {
    let service_map = SERVICE_MAP.clone();
    let service_map  = service_map.lock().unwrap().clone();


    let service_config = service_map.get(&service);

    if service_config.is_none() {
        return Err(String::from("No service specified for service"));
    }
    let service_config = service_config.unwrap();

    let api_map = API_MAP.clone();
    let api_map  = api_map.lock().unwrap().clone();
    let api_config = api_map.get(&format!("{}-{}", service, api));
    if api_config.is_none() {
        return Err(String::from("No service api specified for service"));
    }

    let api_config = api_config.unwrap();

    let full_url_path = format!("{}/{}", service_config.host, api_config.path);

    let request_config = RequestConfig {
        url: full_url_path,
        method: api_config.method.clone(),
        content_type: api_config.content_type.clone(),
    };

    let response = request_config.do_request(params, headers).await;

    if let Ok(response) = response {
        let data : Value = response.json().await.unwrap();
        return Ok(data);
    }
    Err(String::from(response.err().unwrap().to_string()))
}

pub async fn do_request1(
    service: String,
    api: String,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<String, String> {
    Err(String::from(service))
}
