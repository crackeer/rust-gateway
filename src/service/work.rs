use reqwest::{self};
use std::fs::{File};
use std::io::Write;
use std::path::Path;

// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Welcome;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Welcome = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Work {
    base_url: String,
    initial: Initial,
    model: Model,
    observers: Vec<Observer>,
    panorama: Panorama,
    picture_url: String,
    title_picture_url: String,
    vr_code: String,
    vr_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Initial {
    flag_position: Vec<Option<serde_json::Value>>,
    fov: i64,
    heading: i64,
    latitude: f64,
    longitude: f64,
    pano: i64,
    pano_index: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    file_url: String,
    material_base_url: String,
    material_textures: Vec<String>,
    modify_time: String,
    #[serde(rename = "type")]
    model_type: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Observer {
    accessible_nodes: Vec<i64>,
    floor_index: i64,
    index: i64,
    offset_point_count: i64,
    position: Vec<f64>,
    quaternion: Quaternion,
    standing_position: Vec<f64>,
    visible_nodes: Vec<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Panorama {
    count: i64,
    list: Vec<PanoramaItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PanoramaItem {
    back: String,
    derived_id: i64,
    down: String,
    front: String,
    index: i64,
    left: String,
    right: String,
    tiles: Vec<i64>,
    up: String,
}

impl Work {
    fn with_base_url(&self, suffix: &str) -> String {
        let mut full_url = String::from(&self.base_url);
        full_url.push_str(suffix);
        return full_url;
    }
    fn with_model_base_url(&self, suffix: &str) -> String {
        let mut full_url = String::from(&self.model.material_base_url);
        full_url.push_str(suffix);
        return full_url;
    }
    fn get_download_list(&self) -> Vec<(String, String)> {
        let mut download: Vec<(String, String)> = Vec::new();
        for item in self.panorama.list.iter() {
            download.push((self.with_base_url(&item.right), item.right.clone()));
            download.push((self.with_base_url(&item.left), item.left.clone()));
            download.push((self.with_base_url(&item.front), item.front.clone()));
            download.push((self.with_base_url(&item.back), item.back.clone()));
            download.push((self.with_base_url(&item.up), item.up.clone()));
            download.push((self.with_base_url(&item.down), item.down.clone()));
        }
        download.push((self.picture_url.clone(), String::from("picture.jpg")));
        download.push((self.title_picture_url.clone(), String::from("title_picture.jpg")));
        download.push((self.with_base_url(&self.model.file_url), self.model.file_url.clone()));

        for item in self.model.material_textures.iter() {
            download.push((self.with_base_url(&self.with_model_base_url(&item)), self.with_model_base_url(&item)))
        }
        return download;
    }
}


pub async fn download_work_to(work: &Work, path: &Path) {
    let download: Vec<(String, String)> = work.get_download_list();

    for item in download.iter() {
        _ = do_download(item.0.clone(), path.join(item.1.clone()).to_str().unwrap()).await;
    }
}

async fn do_download(url: String, dest: &str) -> Result<(), String> {
    //let resp = reqwest::blocking::get(url);
    let client = reqwest::Client::new();
    let builder = client.get(url);
    let result = builder.send().await;

    if let Err(err) = result {
        return Err(err.to_string());
    }
    let response = result.unwrap();
    let path: &Path = Path::new(dest);
    if let Err(err) = std::fs::create_dir_all(path.parent().unwrap()) {
        return Err(err.to_string());
    }
    let res = File::create(dest);
    if res.is_err() {
        return Ok(());
    }
    let mut buffer = res.unwrap();
    let bytes = response.bytes().await;
    if let Err(err) = buffer.write_all(&bytes.unwrap().to_vec()) {
        return Err(err.to_string());
    }
    //resp.bytes()
    Ok(())
}
