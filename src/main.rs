mod service_api;
mod handler;
mod container;

use axum::{
    routing::{get, post, any},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{Path, Query, Extension},
    http::request::Parts,
};
use std::io::{Error,ErrorKind};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;
use reqwest;
use container::pool::establish_mysql_connection;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = establish_mysql_connection().await;
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler::test::root))
        // `POST /users` goes to `create_user`
        .route("/users", post(handler::test::create_user))
        .route("/relay/:service/:api", any(handler::proxy::relay))
        .route("/files", get(handler::test::md_list))
        .route("/mysql", get(handler::test::fetch_myqsl_data))
        .route("/http", get(handler::test::http_request)).layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
