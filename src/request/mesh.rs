use super::define::Response;
use super::request::do_request;
use crate::request::define::RouterRequestCell;
use reqwest::Error;
use serde_json::Value;
use std::collections::HashMap;
use tokio::spawn;
use tokio::sync::mpsc;

pub async fn do_multi_request(
    wrappers: &Vec<RouterRequestCell>,
    headers : &Option<HashMap<String, String>>,
) -> Result<HashMap<String, Response>, Error> {
    let mut response: HashMap<String, Response> = HashMap::new();
    let mut childs = vec![];
    let (tx, mut rx) = mpsc::channel(32);
    let mut total = wrappers.len();
    for req in wrappers.iter() {
        let parts : Vec<&str>= req.api.split('/').collect();
        let service = parts[0].to_string();
        let api = parts[1].to_string();
        let headers  = headers.clone();
        let tmp = tx.clone();
        let params =  req.params.clone();
        let c = spawn(async move {
            let result = do_request(service, api, params, headers).await;
            println!("Request End{}", result.is_err());
            if let Err(err) = tmp.clone().send(result).await {
                println!("request error{}", err.to_string())
            }
        });
        childs.push(c);
    }
    
    while total > 0{
        if let Some(message) = rx.recv().await {
            total = total-1;
            println!("{}", "recv message");
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
    }
    
    Ok(response)
}

pub async fn do_mesh_request(
    cells: Vec<Vec<RouterRequestCell>>,
    _params: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<HashMap<String, Response>, String> {
    let mut response: HashMap<String, Response> = HashMap::new();

    for cell in cells {
        let result = do_multi_request(&cell, &headers).await;
        if let Ok(res) = result {
            for (key, value) in res.iter() {
                response.insert(key.clone(), value.clone());
            }
        }
    }

    Ok(response)
}
