use std::io::Error;
use std::{
    fs::{read_dir},
};

pub fn get_md_list(dir: String) -> Result<Vec<String>, Error>  {

    let data = read_dir(dir.clone().to_string());
    if data.is_err() {
        return Err(data.err().unwrap())
    }

    let mut dir_vec: Vec<String> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    dir_vec.push(dir.clone().to_string());

   
    let mut cur_index: usize = 0;
    while cur_index < dir_vec.len() {
        let entry = read_dir(dir_vec.get(cur_index).unwrap().to_string());
        if let Ok(data) = entry {
            for item in data.into_iter() {
                if let Ok(data_entry) = item {
                    if let Ok(abc) = data_entry.metadata() {
                        if abc.is_dir() {
                            dir_vec.push(data_entry.path().to_str().unwrap().clone().to_string());
                        } else {
                            let file = data_entry.path().to_str().unwrap().to_string();
                            if file.ends_with(".md") {
                                list.push(file);
                            }
                        }
                    }
                }
            }
        }
        cur_index = cur_index + 1;
    }
    println!("{}", list.join(","));
    Ok(list)
}