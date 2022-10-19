use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use super::define::Service;

pub fn read_service_list(path : String) -> Option<HashMap<String, Service>> {
    println!("path is:{}", path);
    let result = File::open(path);
    let mut content = String::from("");
    if result.is_ok() {
        let mut file = result.unwrap();
        if  file.read_to_string(&mut content).is_err() {
           return None
        }

        let decoded: HashMap<String, Service> = toml::from_str(&content).unwrap();
        println!("connent is {}", content);
        Some(decoded)
    } else {
        println!("{}", result.err().unwrap());
        None
    }
}



