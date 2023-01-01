use super::define::{Router, Service, ServiceAPIFactory, API};
use crate::model::gateway::{router, service, service_api, prelude::{Service as ServiceEntity}};
use axum::async_trait;
use core::panic;
use sea_orm::{Database, DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use sea_orm::{Condition};

#[allow(dead_code)]
pub struct MySqlFactory {
    pool: DatabaseConnection,
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
        let pool = Database::connect(&dsn).await;
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
    async fn get_service_list(&self, env: String) -> Option<HashMap<String, Service>> {
        let list = service::Entity::find().filter(Condition::all().add(service::Column::Env.eq(env.clone()))).all(&self.pool).await;
        if let Ok(data) = list {
            for item in data.iter() {
                println!("{}", item.name);
            }
        }
        None
    }
    async fn get_api_list(&self, _service: String) -> Option<HashMap<String, API>> {
        
        None
    }

    async fn get_router_list(&self) -> Option<HashMap<String, Router>> {
        None
    }
}
