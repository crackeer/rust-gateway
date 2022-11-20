use crate::util::file as util_file;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Debug, Clone)]
pub struct Response {
    pub name : String,
    pub data: Option<Value>,
    pub code: u64,
    pub message: String,
    pub cost: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Router {
    pub config: Vec<Vec<RouterRequestCell>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RouterRequestCell {
    pub name: String,
    pub api: String,
    pub params: Option<Value>,
    pub recovery: Option<bool>,
}

pub trait ServiceAPIFactory {
    fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>>;
    fn get_api_list(&self, service: String) -> Option<HashMap<String, API>>;
    fn get_router_list(&self) -> Option<HashMap<String, Router>>;
}

pub struct FileFactory {
    pub service_path: String,
    pub api_path: String,
    pub router_path: String,
}

impl FileFactory {
    pub fn new(service_path: String, api_path: String, router_path: String) -> FileFactory {
        return FileFactory {
            service_path: service_path,
            api_path: api_path,
            router_path: router_path,
        };
    }
}

impl ServiceAPIFactory for FileFactory {
    fn get_api_list(&self, service: String) -> Option<HashMap<String, API>> {
        let full_path = format!("{}/{}.toml", self.api_path, service);
        let content = util_file::read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, API> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
    fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>> {
        let full_path = format!("{}/{}.toml", self.service_path, env);
        let content = util_file::read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, Service> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
    fn get_router_list(&self) -> Option<HashMap<String, Router>> {
        //println!("{}", self.router_path);
        let file_list = util_file::get_file_list(self.router_path.clone(), String::from(".toml"));
        //println!("{}", file_list.join(","));
        let mut response: HashMap<String, Router> = HashMap::new();

        for file in file_list {
            if let Ok(content) = util_file::read_file(file.as_str()) {
                let decoded: Router = toml::from_str(&content).unwrap();
                response.insert(trim_router_path(&file, &self.router_path, ".toml"), decoded);
            }
        }
        Some(response)
    }
}

fn trim_router_path(path : &String, prefix: &str, suffix : &str) ->String  {
    if path.len() < 1 {
        return String::new();
    }
    //println!("{}-{}-{}", path,prefix, suffix);
    path.clone().strip_prefix(prefix).unwrap().strip_suffix(suffix).unwrap().to_string().replace("\\", "/")
}
