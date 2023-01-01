//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "service")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub namespace: String,
    pub sign_config: Option<String>,
    pub env: Option<String>,
    pub host: String,
    pub timeout: u32,
    pub description: String,
    pub config: ConfigValue,
    pub create_time: DateTime,
    pub modify_time: DateTime,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ConfigValue {
    pub data_key: String,
    pub code_key: String,
    pub success_code: String,
    pub message_key: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
