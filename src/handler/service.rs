use crate::request::define::{FileFactory, ServiceAPIFactory};
use axum::{
    extract::{ Path},
    response::IntoResponse
};
use std::{collections::HashMap};

pub async fn get_service_list(Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {

    let factory  = FileFactory::new(String::from("./config/service"), String::from("./config/api"));
    axum::Json(factory.get_service_list(params.get("env").unwrap().to_string()))
}