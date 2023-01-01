use super::define::{Router, Service, ServiceAPIFactory, API};
use crate::util::file as util_file;
use axum::async_trait;
use serde_json;
use std::collections::HashMap;
use tracing::error;

pub struct FileFactory {
    pub service_path: String,
    pub api_path: String,
    pub router_path: String,
}

impl FileFactory {
    pub async fn new(service_path: String, api_path: String, router_path: String) -> FileFactory {
        return FileFactory {
            service_path: service_path,
            api_path: api_path,
            router_path: router_path,
        };
    }
}

#[async_trait]
impl ServiceAPIFactory for FileFactory {
    async fn get_api_list(&self, service: String) -> Option<HashMap<String, API>> {
        let full_path = format!("{}/{}.toml", self.api_path, service);
        let content = util_file::read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, API> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
    async fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>> {
        let full_path = format!("{}/{}.toml", self.service_path, env);
        let content = util_file::read_file(full_path.as_str());
        if content.is_ok() {
            let decoded: HashMap<String, Service> = toml::from_str(&content.unwrap()).unwrap();
            Some(decoded)
        } else {
            None
        }
    }
    async fn get_router_list(&self) -> Option<HashMap<String, Router>> {
        //println!("{}", self.router_path);
        let file_list = util_file::get_file_list(self.router_path.clone(), String::from(".json"));
        //println!("{}", file_list.join(","));
        let mut response: HashMap<String, Router> = HashMap::new();

        for file in file_list {
            match util_file::read_file(file.as_str()) {
                Err(err) => {
                    error!("{}", err);
                }
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(decoded) => {
                        response
                            .insert(trim_router_path(&file, &self.router_path, ".json"), decoded);
                    }
                    Err(err) => error!("json decode {} error {}", &self.router_path, err),
                },
            }
        }
        Some(response)
    }
}

fn trim_router_path(path: &String, prefix: &str, suffix: &str) -> String {
    if path.len() < 1 {
        return String::new();
    }
    //println!("{}-{}-{}", path,prefix, suffix);
    path.clone()
        .strip_prefix(prefix)
        .unwrap()
        .strip_suffix(suffix)
        .unwrap()
        .to_string()
        .replace("\\", "/")
}
