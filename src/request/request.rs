use super::define::{Service, API};
use crate::container::timer::{API_MAP, SERVICE_MAP};
use reqwest::{blocking::Response, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, future::Future, sync::Arc};

pub struct RequestConfig {
    pub url: String,
    pub method: String,
    pub content_type: Option<String>,
}

impl RequestConfig {
    pub fn cat(
        &self,
        value: Option<Value>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Value, String> {
        let client = reqwest::blocking::Client::new();
        let mut builder: reqwest::blocking::RequestBuilder = client.get(&self.url);
        let content_type = self.content_type.as_ref().unwrap();
        if self.method == "POST" {
            builder = client.post(&self.url);
            if let Some(params) = value {
                if content_type == "application/json" {
                    builder = builder.json(&params);
                }
            }
        }
        //return Ok(String::from("Simple RequestConfig"));
        let response = builder.send();
        if let Ok(real) = response {
            let value : Value = real.json().unwrap();
            return Ok(value)
        }
        Err(response.err().unwrap().to_string())
       
    }
}


pub async fn do_request(
    service: String,
    api: String,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<Value, String> {
    let service_map = SERVICE_MAP.try_lock().unwrap();
    let service_config = service_map.get(&service);
    if service_config.is_none() {
        return Err(String::from("No service specified for service"));
    }
    let service_config = service_config.unwrap();

    let api_map = API_MAP.try_lock().unwrap();
    let api_config = api_map.get(&api);
    if api_config.is_none() {
        return Err(String::from("No service specified for service"));
    }

    let api_config = api_config.unwrap();

    let full_url_path = format!("{}/{}", service_config.host, api_config.path);
    /*
    let response = run_test_request(full_url_path, api_config.method.clone(), api_config.content_type.clone(), headers, params);
    if let Ok(response) = response {
        let value : Value = response.json().unwrap();
        return Ok(value)
    }
    Err(String::from(response.err().unwrap().to_string()))
     */

    
    let request_config = RequestConfig {
        url: full_url_path,
        method: api_config.method.clone(),
        content_type: api_config.content_type.clone(),
    };

    let response = request_config.cat(params, headers);

    if let Ok(response) = response {
        return Ok(response)
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
