use std::fs::File;
use std::io::Read;
use super::define::APIConfig;

pub fn read_config(path : String) -> Option<Box<APIConfig>> {
    println!("path is:{}", path);
    let result = File::open(path);
    let mut content = String::from("");
    if result.is_ok() {
        let mut file = result.unwrap();
        if  file.read_to_string(&mut content).is_err() {
           return None
        }

        let decoded: APIConfig = toml::from_str(&content).unwrap();
        println!("connent is {}", content);
        Some(Box::new(decoded))
    } else {
        println!("{}", result.err().unwrap());
        None
    }
}



