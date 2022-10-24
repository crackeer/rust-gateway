use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub host: String,
    pub timeout: u32,
    pub data_key: String,
    pub code_key: String,
    pub success_code_key: String,
    pub error_message_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct API {
    pub path: String,
    pub method: String,
    pub content_type: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Response {
    pub data: Option<Value>,
    pub code: usize,
    pub message: String,
    pub cost:usize
}

pub trait ServiceAPIFactory {
    fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>>;
    fn get_api_list(&self, service: String) -> Option<HashMap<String, API>>;
}

pub struct FileFactory {
    pub service_path: String,
    pub api_path: String,
}

impl FileFactory {
    pub fn new(service_path: String, api_path: String) -> FileFactory {
        return FileFactory {
            service_path: service_path,
            api_path: api_path,
        };
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    //println!("{}", path);
    let mut result = File::open(path)?;
    let mut content = String::from("");
    result.read_to_string(&mut content)?;
    Ok(content)
}

impl ServiceAPIFactory for FileFactory {
    fn get_api_list(&self, service: String) -> Option<HashMap<String, API>> {
        let full_path = format!("{}/{}.toml", self.api_path, service);
        let content = read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, API> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
    fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>> {
        let full_path = format!("{}/{}.toml", self.service_path, env);
        let content = read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, Service> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
}
