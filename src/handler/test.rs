use crate::container::timer::APIMAP;
use crate::service_api::api::get_md_list;
use axum::extract::Extension;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use reqwest;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

pub async fn md_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = params.get("dir");
    if path.is_some() {
        let data = get_md_list(path.unwrap().to_string());
        if data.is_ok() {
            return (StatusCode::CREATED, Json(data.unwrap()));
        }
    }
    (StatusCode::CREATED, Json(vec![]))
}

pub async fn http_request(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let resp = reqwest::get("https://httpbin.org/ip").await;

    let mut test = HashMap::new();
    test.insert(String::from("test"), String::from("test"));

    if resp.is_err() {
        return (StatusCode::CREATED, Json(test));
    }

    let text = resp.unwrap().json::<HashMap<String, String>>().await;

    (StatusCode::CREATED, Json(text.unwrap()))
}

// the input to our `create_user` handler
// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Actor {
    pub actor_id: u32,
    pub first_name: String,
}
#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct AAA(HashMap<String, String>);

pub async fn fetch_mysql_data(Extension(pool): Extension<Pool<MySql>>) -> impl IntoResponse {
    let list = sqlx::query_as::<_, Actor>(r#"select * from actor"#)
        .fetch_all(&pool)
        .await
        .unwrap();
    (StatusCode::OK, Json(list))
}

/* 
pub async fn get_actor(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let mut the_map = APIMAP.try_lock().unwrap();

    let i1: u32 = 200;
    let data = the_map.get(&i1).unwrap().clone();
    axum::Json(data)
}
*/
