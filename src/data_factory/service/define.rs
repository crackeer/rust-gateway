use axum::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub host: String,
    pub timeout: u32,
    pub data_key: String,
    pub code_key: String,
    pub success_code: String,
    pub message_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct API {
    pub path: String,
    pub method: String,
    pub content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Router {
    pub config: Vec<Vec<RouterRequestCell>>,
    pub response: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DBRouter {
    pub config: String,
    pub response: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterRequestCell {
    pub name: String,
    pub api: String,
    pub params: Option<Value>,
    pub recovery: Option<bool>,
}

#[async_trait]
pub trait ServiceAPIFactory {
    async fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>>;
    async fn get_api_list(&self, service: String) -> Option<HashMap<String, API>>;
    async fn get_router_list(&self) -> Option<HashMap<String, Router>>;
}