use std::fs::File;
use std::io::Read;
use std::{
    fs::{read_dir},
};

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut result = File::open(path)?;
    let mut content = String::from("");
    result.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_file_list(dir: String, ext: String) -> Vec<String> {
    let mut dir_vec: Vec<String> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    dir_vec.push(dir);

    let mut cur_index: usize = 0;

    while cur_index < dir_vec.len() {

        let cur_dir = dir_vec.get(cur_index);

        if cur_dir.is_none() {
            break;
        }

        let entry = read_dir(cur_dir.unwrap().to_string());
        cur_index = cur_index + 1;
        if entry.is_err() {
            continue;
        }

        for item in entry.unwrap().into_iter() {
            if item.is_err() {
                continue;
            }
            let f = item.unwrap();
            let name = f.path().to_str().unwrap().to_string();
            if let Ok(meta) = f.metadata() {
                if meta.is_dir() {
                    dir_vec.push(name);
                } else {
                    if name.ends_with(&ext) {
                        list.push(name);
                    }
                }
            }
        }
    }
    list
}
