use crate::request::define::{ServiceAPIFactory, Service, API};

/* 
use sqlx::{MySql, Pool};
*/

use std::collections::HashMap;
use std::result;
use std::sync::{Arc, Mutex};
use tokio::{time, spawn};

lazy_static! {
    pub static ref SERVICE_MAP: Arc<Mutex<HashMap<String, Service>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref API_MAP: Arc<Mutex<HashMap<String, API>>> = Arc::new(Mutex::new(HashMap::new()));
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

pub async fn load_service_api(factory: Arc<impl ServiceAPIFactory>, env : String) {
    let mut interval = time::interval(time::Duration::from_secs(1));

    loop {
        interval.tick().await;
        println!("load_service_api coming");

        let service_list = factory.get_service_list(env.clone());
        let result = SERVICE_MAP.try_lock();
        if result.is_err() {
            println!("load_service_api failed, err = {}", result.err().unwrap());
            continue;
        }
        let mut service_map = result.unwrap();
        let mut api_map = API_MAP.try_lock().unwrap();
        if let Some(service_list) = service_list {
            for (key, item) in service_list.iter() {
                service_map.insert(key.clone(), item.clone());
                if let Some(tmp_api_map) = factory.get_api_list(key.clone()) {
                    for (key1, item1) in tmp_api_map.iter() {
                        api_map.insert(format!("{}-{}", key, key1), item1.clone());
                    }
                }
            }
        }
        spawn(check());
        //check();
        /* 
        let result = SERVICE_MAP.clone();
       
        let service_map_tmp  = result.try_lock().unwrap();
    
        for (key, value) in service_map_tmp.iter() {
            println!("getget:{}", key);
        }*/
        
    }
}

async fn check() {
    let service_map = SERVICE_MAP.clone();
    let service_map  = service_map.lock().unwrap().clone();
    for (key, value) in service_map.iter() {
        println!("getget:{}", key);
    }
}

