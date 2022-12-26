

struct config {
    pub port : usize,
    pub driver : String,
    pub file : file_part,
    pub mysql : mysql_part,
    pub log: log_part,
}

pub struct file_part {
   pub  dir : String,
}

pub struct  mysql_part {
    pub host : String,
    pub user : String,
    pub password : String,
    pub database : String,
    pub env : String,
}

pub struct log_part {
    pub dir : String,
    pub level : String,
}