use crate::util::file as util_file;
use axum::async_trait;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;
use tracing::error;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Service {
    pub host: String,
    pub timeout: u32,
    pub data_key: String,
    pub code_key: String,
    pub success_code: String,
    pub message_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct API {
    pub path: String,
    pub method: String,
    pub content_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Router {
    pub config: Vec<Vec<RouterRequestCell>>,
    pub response: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct DBRouter {
    pub config: String,
    pub response: String,
}



#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
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

#[allow(dead_code)]
pub struct MySqlFactory {
    pool: Pool<MySql>,
}

#[allow(dead_code)]
impl MySqlFactory {
    pub async fn new(
        user: String,
        password: String,
        host: String,
        database: String,
    ) -> MySqlFactory {
        let dsn = format!("mysql://{}:{}@{}/{}", user, password, host, database);
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&dsn)
            .await;
        if let Err(err) = pool {
            panic!("Couldn't connect to MySQL：{}", err.to_string())
        }
        return MySqlFactory {
            pool: pool.unwrap(),
        };
    }
}

#[async_trait]
impl ServiceAPIFactory for MySqlFactory {
    async fn get_api_list(&self, _service: String) -> Option<HashMap<String, API>> {
        let list = sqlx::query_as::<_, API>(r#"select * from service_api"#)
            .fetch_all(&self.pool)
            .await;
        let mut data: HashMap<String, API> = HashMap::new();
        if let Ok(list) = list {
            for api in list.into_iter() {
                data.insert(api.path.clone(), api.clone());
            }
            return Some(data);
        }
        None
    }
    async fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>> {
        let list = sqlx::query_as::<_, Service>(r#"select * from service_api"#)
            .fetch_all(&self.pool)
            .await;
        let mut data: HashMap<String, Service> = HashMap::new();
        if let Ok(list) = list {
            for api in list.into_iter() {
                data.insert(api.host.clone(), api.clone());
            }
            return Some(data);
        }
        None
    }
    async fn get_router_list(&self) -> Option<HashMap<String, Router>> {
        let list = sqlx::query_as::<_, DBRouter>(r#"select * from router"#)
            .fetch_all(&self.pool)
            .await;
        let mut data: HashMap<String, Router> = HashMap::new();
        if let Ok(list) = list {
            for api in list.into_iter() {
                //data.insert(String::from("simple"), api.clone());
            }
            return Some(data);
        }
        None
    }
}
