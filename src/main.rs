mod container;
mod handler;
mod model;
mod request;
mod service_api;
mod util;
mod data_factory;

#[macro_use]
extern crate lazy_static;
use axum::{
    //extract::Extension,
    routing::{any, get, post},
    Router,
};
//use container::pool::establish_mysql_connection;
use container::api::load_service_api;
use container::config::{Config, DRIVER_FILE, DRIVER_MYSQL, LogPart};
use toml;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};
use util::file as util_file;
use data_factory::service::file::FileFactory;

fn init_tracing_log(log_part: LogPart){
    // 输出到控制台中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let file_appender = rolling::never(log_part.dir.as_str(), log_part.filename.as_str());
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    Registry::default()
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();
}

fn init_config() -> Config {
    let data = match util_file::read_file("./etc/entry.toml") {
        Ok(data) => data,
        Err(err) => panic!("read entry.toml error:{}", err),
    };
    match toml::from_str(&data) {
        Ok(config) => config,
        Err(err) => panic!("decode error:{}", err),
    }
}

fn init_api_factory(config : &Config){
    if config.driver == DRIVER_FILE {
        let factory = FileFactory::new(
            config.file.service_dir.clone(),
            config.file.api_dir.clone(),
            config.file.router_dir.clone()
        );
        tokio::spawn(load_service_api(Arc::new(factory), config.file.router_dir.clone()));
    } else if config.driver == DRIVER_MYSQL {
        todo!();
    }
}

#[tokio::main]
async fn main() {
    let config = init_config();
    init_tracing_log(config.log.clone());
    init_api_factory(&config);
    

    //tokio::spawn(load_api(Arc::new(pool.to_owned())));
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler::test::root))
        // `POST /users` goes to `create_user`
        .route("/users", post(handler::test::create_user))
        .route("/service/:env", any(handler::service::get_service_list))
        .route("/routers", any(handler::service::get_router_list))
        .route("/relay/:service/:api", any(handler::proxy::relay))
        .route("/files", get(handler::test::md_list))
        .route("/mysql", get(handler::test::fetch_mysql_data))
        .route("/http", any(handler::test::http_request))
        .fallback(any(handler::mesh::mesh));
    //.layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
