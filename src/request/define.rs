use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    host: String,
    timeout: u32,
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    pub path: String,
    pub method: String,
    pub id: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct APIConfig {
    service: Service,
    api_list: Vec<Api>,
}
