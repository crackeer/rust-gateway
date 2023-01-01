use crate::data_factory::service::define::{Router, Service, ServiceAPIFactory, API};
use tracing::{info, error};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time;

lazy_static! {
    pub static ref SERVICE_MAP: Arc<Mutex<HashMap<String, Service>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref API_MAP: Arc<Mutex<HashMap<String, API>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ROUTER_MAP: Arc<Mutex<HashMap<String, Router>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

/*
pub async fn load_api_by_mysql(arc_pool: Arc<Pool<MySql>>) {
    let mut interval = time::interval(time::Duration::from_secs(5));

    loop {
        interval.tick().await;
        println!("2333");

        let list = sqlx::query_as::<_, Actor>(r#"select * from actor"#)
            .fetch_all(arc_pool.as_ref())
            .await
            .unwrap();

        let mut the_map = APIMAP.try_lock().unwrap();
        for item in list.into_iter() {
            the_map.insert(item.actor_id, item);
        }

        let i1: u32 = 200;
        println!("{:?}", the_map.get(&i1).unwrap());
    }
}
*/

pub async fn load_service_api(factory: Arc<impl ServiceAPIFactory>, env: String) {
    let mut interval = time::interval(time::Duration::from_secs(5));

    loop {
        interval.tick().await;
        info!("load_service_api");
        let service_list = factory.get_service_list(env.clone()).await;
        
        if let Some(service_list) = service_list {
            for (key, item) in service_list.iter() {
                SERVICE_MAP.lock().unwrap().insert(key.clone(), item.clone());
                let tmp =  factory.get_api_list(key.clone()).await.clone();
                if let Some(tmp_api_map) = tmp {
                    for (key1, item1) in tmp_api_map.iter() {
                        API_MAP.lock().unwrap().insert(format!("{}-{}", key, key1), item1.clone());
                    }
                }
            }
        }
        let router_list = factory.get_router_list().await;
       
        if let Some(list) = router_list {
            for (key, item) in list.iter() {
                println!("{}", key.clone());
                ROUTER_MAP.lock().unwrap().insert(key.clone(), item.clone());
            }
        }
    }
}

pub fn get_service(name: &String) -> Option<Service> {
    let service_map = SERVICE_MAP.clone();
    let locker = service_map.lock().unwrap();
    if let Some(data) = locker.get(&format!("{}", name)) {
        return Some(data.clone());
    }
    None
}

pub fn get_service_api(name: &String, api: &String) -> Option<API> {
    let api_map = API_MAP.clone();
    let locker = api_map.lock().unwrap();
    if let Some(data) = locker.get(&format!("{}-{}", name, api)) {
        return Some(data.clone());
    }
    None
}

pub fn get_router_config(path: &String) -> Option<Router> {
    let router_map = ROUTER_MAP.clone();
    let locker = router_map.lock().unwrap();
    if let Some(data) = locker.get(&format!("{}", path)) {
        return Some(data.clone());
    }
    None
}
