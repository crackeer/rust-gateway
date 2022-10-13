use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

pub async fn establish_mysql_connection() -> Pool<MySql> {
    let database_url = String::from("mysql://root:12345678@localhost/sakila");
     let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await;
       // .connect("mysql://sulin:databenD!9@localhost:3306/shorten_db").await;
    pool.unwrap()
}