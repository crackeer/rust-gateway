use super::define::Response;
use super::request::do_request;
use crate::request::define::RouterRequestCell;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::spawn;
use tokio::sync::mpsc;

pub async fn do_multi_request(
    wrappers: &Vec<RouterRequestCell>,
) -> Result<HashMap<String, Response>, Error> {
    //let mut response: Arc<Mutex<HashMap<String, APIResponse>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut response: HashMap<String, Response> = HashMap::new();
    let mut childs = vec![];
    let (tx, mut rx) = mpsc::channel(32);
    for req in wrappers.iter() {
        let service = req.api;
        let api = req.api;
        let response_clone = response.clone();
        let headers: HashMap<String, String> = HashMap::new();
        let c = spawn(async move {
            let result = do_request(service, api, req.params, Some(headers)).await;
            tx.clone().send(result);
            /*
            let mut vs = response_clone.lock().unwrap();
            if let Ok(res) = do_request(service, api, req.params, Some(headers)).await {
                vs.insert(req.name.clone(), res)
            } else {
                vs.insert(req.name.clone(), APIResponse{
                    message: "error".to_string(),
                    data : Some(Value::from(String::from("simple error"))),
                    code : 0,
                    cost : 0,
                })
            }
            */
        });
        childs.push(c);
    }
    while let Some(message) = rx.recv().await {
        if let Ok(res) = message {
            response.insert(String::from("Some"), res);
        } else {
            response.insert(
                String::from("Error"),
                Response {
                    message: "error".to_string(),
                    data: Some(Value::from(String::from("simple error"))),
                    code: 0,
                    cost: 0,
                },
            );
        }
    }

    Ok(response)
}

pub async fn do_mesh_request(
    cells: Vec<Vec<RouterRequestCell>>,
    params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<HashMap<String, Response>, String> {
    let mut response: HashMap<String, Response> = HashMap::new();

    for cell in cells {
        let result = do_multi_request(&cell).await;
        if let Ok(res) = result {
            for (key, value) in result.iter() {
                response.insert(key, value);
            }
        }
    }

    Ok(response)
}
