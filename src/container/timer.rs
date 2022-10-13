use crate::model::define::Actor;
//use crate::request::define::Api;

use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time;

lazy_static! {
    pub static ref APIMAP: Arc<Mutex<HashMap<u32, Actor>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn load_api(arc_pool: Arc<Pool<MySql>>) {
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
