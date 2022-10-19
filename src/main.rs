mod container;
mod handler;
mod request;
mod service_api;
mod model;

#[macro_use]
extern crate lazy_static;
use axum::{
    extract::Extension,
    routing::{any, get, post},
    Router,
};
use container::pool::establish_mysql_connection;
use container::timer::{load_api};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    //let pool = establish_mysql_connection().await;
    
    //tokio::spawn(load_api(Arc::new(pool.to_owned())));
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler::test::root))
        // `POST /users` goes to `create_user`
        .route("/users", post(handler::test::create_user))
        .route("/service/:env",  any(handler::service::get_service_list))
        .route("/relay/:service/:api", any(handler::proxy::relay))
        .route("/files", get(handler::test::md_list))
        .route("/mysql", get(handler::test::fetch_mysql_data))
        .route("/http", get(handler::test::http_request))
        .route("/actor", get(handler::test::get_actor));
        //.layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    print!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
