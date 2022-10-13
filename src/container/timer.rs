use crate::request::define::Api;
use std::{collections::HashMap};
use tokio::time;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref APIMAP: Arc<Mutex<HashMap<String, Api>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn print() {
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("2333");
        APIMAP.try_lock().unwrap().insert(
            String::from("Simple"),
            Api {
                path: String::from("ss"),
                method: String::from("POST"),
                id: String::from("ss"),
                name : Some(String::from("NAME")),
            },
        );
    }
}
