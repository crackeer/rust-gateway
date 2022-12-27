
#[allow(dead_code)]
pub struct Config {
    pub port : usize,
    pub driver : String,
    pub file : FilePart,
    pub mysql : MysqlPart,
    pub log: LogPart,
}

pub struct FilePart {
   pub  dir : String,
}

pub struct  MysqlPart {
    pub host : String,
    pub user : String,
    pub password : String,
    pub database : String,
    pub env : String,
}

pub struct LogPart {
    pub dir : String,
    pub level : String,
}