use axum::async_trait;
use core::panic;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;
use super::define::{ServiceAPIFactory, Service, API, Router};
#[allow(dead_code)]
pub struct MySqlFactory {
    pool: Pool<MySql>,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct DBRouter {
    pub config: String,
    pub response: String,
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
