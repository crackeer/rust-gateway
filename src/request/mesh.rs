use super::request::do_request;
use crate::data_factory::service::define::RouterRequestCell;
use crate::util::json::extract_value;
use reqwest::Error;
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::spawn;
use tokio::sync::mpsc;

pub async fn do_multi_request(
    wrappers: &Vec<RouterRequestCell>,
    params: &Value,
    headers: &HashMap<String, String>,
) -> Result<HashMap<String, Option<Value>>, Error> {
    let mut response: HashMap<String, Option<Value>> = HashMap::new();
    let mut childs = vec![];
    let (tx, mut rx) = mpsc::channel(32);
    let mut total = wrappers.len();
    for req in wrappers.iter() {
        let parts: Vec<&str> = req.api.split('/').collect();
        let service = parts[0].to_string();
        let api = parts[1].to_string();
        let headers = headers.clone();
        let tmp = tx.clone();
        let mut real_params: Value = json!({});
        if let Some(value) = extract_value(&params, &req.params.clone().unwrap()) {
            real_params = value
        }
        let name = req.name.clone();
        let c = spawn(async move {
            let result = do_request(service, api, &real_params, &headers, name).await;
            if let Err(err) = tmp.clone().send(result).await {
                println!("request error{}", err.to_string())
            }
        });
        childs.push(c);
    }

    while total > 0 {
        if let Some(message) = rx.recv().await {
            total = total - 1;
            println!("{}", "recv message");
            response.insert(message.name.clone(), message.data);
        }
    }

    Ok(response)
}

pub async fn do_mesh_request(
    cells: Vec<Vec<RouterRequestCell>>,
    params: &Value,
    headers: &HashMap<String, String>,
) -> Result<Value, String> {
    let mut input: Value = json!({});
    input
        .as_object_mut()
        .unwrap()
        .insert(String::from("input"), params.clone());

    for cell in cells {
        let result = do_multi_request(&cell, &input, headers).await;
        if let Ok(res) = result {
            for (key, value) in res.iter() {
                if let Some(val) = value {
                    input
                        .as_object_mut()
                        .unwrap()
                        .insert(key.clone(), val.clone());
                }
            }
        }
    }
    Ok(input)
}
