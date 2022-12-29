use crate::request::define::{FileFactory, ServiceAPIFactory};
use axum::{
    extract::{ Path, Query},
    response::IntoResponse
};
use std::{collections::HashMap};

pub async fn get_service_list(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let factory  = FileFactory::new(String::from("./config/service"), String::from("./config/api"), String::from("./config/router"));
    axum::Json(factory.get_service_list(params.get("env").unwrap().to_string()).await)
}

pub async fn get_router_list(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    println!("{}", "Simple");
    let factory  = FileFactory::new(String::from("./config/service"), String::from("./config/api"), String::from("./config/router"));
    axum::Json(factory.get_router_list().await)
}