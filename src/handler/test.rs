
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{Path, Query},
    http::request::Parts,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::service_api::{api::get_md_list};
use reqwest;


// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn proxy(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = params.get("path");
    if path.is_some() {
        let data = api::read_config(path.unwrap().to_string());
        if data.is_some() {
            //println!("Some Data:{}", data);
            (StatusCode::CREATED, Json(data))
        } else {
            (StatusCode::CREATED, Json(data))
        }
    } else {
        (StatusCode::CREATED, Json(None))
    }
    
}

pub async fn md_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = params.get("dir");
    if path.is_some() {
        let data = get_md_list(path.unwrap().to_string());
        if data.is_ok() {
            return (StatusCode::CREATED, Json(data.unwrap()));
        }
    }
    (StatusCode::CREATED, Json(vec![]))
}

pub async fn http_request(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let resp = reqwest::get("https://httpbin.org/ip").await;

    let mut test = HashMap::new();
    test.insert(String::from("test"), String::from("test"));

    if resp.is_err() {
        return  (StatusCode::CREATED, Json(test))
    }

    let text = resp.unwrap().json::<HashMap<String, String>>().await;
        
    (StatusCode::CREATED, Json(text.unwrap()))
}


// the input to our `create_user` handler
// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}