use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub const DRIVER_FILE : &'static str = "file";
#[allow(dead_code)]
pub const DRIVER_MYSQL : &'static str = "mysql";

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub port : usize,
    pub driver : String,
    pub file : FilePart,
    pub mysql : MysqlPart,
    pub log: LogPart,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilePart {
   pub api_dir : String,
   pub service_dir : String,
   pub router_dir : String,
   pub env :String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct  MysqlPart {
    pub host : String,
    pub user : String,
    pub password : String,
    pub database : String,
    pub env : String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogPart {
    pub dir : String,
    pub filename : String,
    pub level : String,
}