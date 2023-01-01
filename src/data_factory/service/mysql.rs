use super::define::{Router, Service, ServiceAPIFactory, API};
use crate::data_factory::service::define::RouterRequestCell;
use crate::model::gateway::{router, service, service_api};
use axum::async_trait;
use core::panic;
use sea_orm::Condition;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use std::collections::HashMap;
use tracing::error;

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
        let list = service::Entity::find()
            .filter(Condition::all().add(service::Column::Env.eq(env.clone())))
            .all(&self.pool)
            .await;
        let mut ret = HashMap::new();
        if let Ok(data) = list {
            for item in data.iter() {
                ret.insert(
                    item.name.clone(),
                    Service {
                        host: item.host.clone(),
                        timeout: item.timeout,
                        success_code: item.config.success_code.clone(),
                        message_key: item.config.message_key.clone(),
                        code_key: item.config.code_key.clone(),
                        data_key: item.config.data_key.clone(),
                    },
                );
            }
        }
        Some(ret)
    }
    async fn get_api_list(&self, service_name: String) -> Option<HashMap<String, API>> {
        let list = service_api::Entity::find()
            .filter(Condition::all().add(service_api::Column::Service.eq(service_name.clone())))
            .all(&self.pool)
            .await;
        let mut ret = HashMap::new();
        if let Ok(data) = list {
            for item in data.iter() {
                ret.insert(
                    item.name.clone(),
                    API {
                        path: item.path.clone(),
                        method: item.method.clone(),
                        content_type: item.content_type.clone(),
                    },
                );
            }
        }
        Some(ret)
    }

    async fn get_router_list(&self) -> Option<HashMap<String, Router>> {
        let list = router::Entity::find().all(&self.pool).await;
        let mut ret = HashMap::new();
        match list {
            Ok(list) => {
                for item in list.iter() {
                    let mut cnf: Vec<Vec<RouterRequestCell>> = vec![];
                    if let Some(config) = &item.config {
                        if let Ok(config) = serde_json::from_str(config.as_str()) {
                            cnf = config
                        }
                    }
                    ret.insert(
                        item.path.clone(),
                        Router {
                            config: cnf,
                            response: item.response.clone(),
                        },
                    );
                }
            }
            Err(err) => {
                error!("{}", err);
            }
        }
        Some(ret)
    }
}
