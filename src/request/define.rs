
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    host: String,
    timeout: u32,
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Api {
    path: String,
    method: String,
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct APIConfig {
    service : Service,
    api_list : Vec<Api>,
}