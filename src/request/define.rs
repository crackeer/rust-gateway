use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Service {
    host: String,
    timeout: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct API {
    pub path: String,
    pub method: String,
    pub content_type : Option<String>,
}