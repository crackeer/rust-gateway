use crate::request::http::read_service_list;
use axum::{
    extract::{ Path},
    response::IntoResponse
};
use std::{collections::HashMap};

pub async fn get_service_list(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    println!("{}", "SSS service list is not available");
    let env  = params.get("env").unwrap();
    let path = format!("./config/service/{}.toml", env);
    let list = read_service_list(path);
    axum::Json(list)
}