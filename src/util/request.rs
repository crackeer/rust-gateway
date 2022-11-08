use serde_json::{json, Value};
use axum::{
    body::{Bytes, HttpBody},
    extract::{FromRequest, Query, RequestParts},
    http::{header::CONTENT_TYPE},
    BoxError,
};
use std::{
    collections::{hash_map::RandomState, HashMap}
};

const CONTENT_TYPE_APPLICATION: &'static str = "application/json";


pub fn extract_header<B>(req: &mut RequestParts<B>) -> HashMap<String, String> where 
B: Send,
B: HttpBody + Send,
B::Data: Send,
B::Error: Into<BoxError>,
{
    let mut header: HashMap<String, String> = HashMap::new();
    for (key, value) in req.headers().iter() {
        let tmp_str = value.to_str().unwrap();
        header.insert(key.to_string(), tmp_str.to_string().clone());
    }
    return header;
}

pub async fn extract_parameter_from_get<B>(req: &mut RequestParts<B>) -> HashMap<String, Value> where 
B: Send,
B: HttpBody + Send,
B::Data: Send,
B::Error: Into<BoxError>,
{
    let mut object : HashMap<String, Value>= HashMap::new();
    let query: Query<HashMap<String, String, RandomState>> = Query::from_request(req).await.unwrap();
    for (key, value) in query.iter() {
        object.insert(String::from(key), Value::String(value.clone()));
    }
    return object
}

async fn extract_parameter_from_post<B>(req: &mut RequestParts<B>) -> HashMap<String, Value> where 
B: Send,
B: HttpBody + Send,
B::Data: Send,
B::Error: Into<BoxError>,
{

    let mut object : HashMap<String, Value>= HashMap::new();
    let content_type_header = req.headers().get(CONTENT_TYPE);
    let content_type = content_type_header.and_then(|value| value.to_str().ok());

    if req.method().to_string().eq("POST") {
        if let Some(content_type) = content_type {
            if content_type == CONTENT_TYPE_APPLICATION {
                let bytes = Bytes::from_request(req).await.unwrap();
                let post_data: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
                if let Value::Object(post_map) = post_data {
                    for (key, value) in post_map.iter() {
                        object.insert(String::from(key), value.clone());
                    }
                }
            }
        }
    }
    return object
}

pub async fn extract_parameter_all<B>(req: &mut RequestParts<B>) -> HashMap<String, Value> where 
B: Send,
B: HttpBody + Send,
B::Data: Send,
B::Error: Into<BoxError>,
{
    let mut get_data: HashMap<String, Value> = extract_parameter_from_get(req).await;
    let post_data: HashMap<String, Value> = extract_parameter_from_post(req).await;
    for (key, value) in post_data.iter() {
        get_data.insert(String::from(key), value.clone());
    }

    return get_data
}

