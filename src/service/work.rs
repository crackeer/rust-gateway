

use std::path::{Path};
use std::fmt::Display;
use std::{
    fs::{self, File},
};
use std::io::Write;
use reqwest::{self};

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

use serde::{Serialize, Deserialize};
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

#[derive(Serialize, Deserialize,Clone, Debug)]
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

pub async fn download_work_to(work : &Work, path : &Path)  {
    let mut download : Vec<(String, String)>= Vec::new();
    for item in work.panorama.list.iter() {
        let mut full_url = String::from(&work.base_url);
        full_url.push_str(&item.right);
        download.push((full_url, path.join(&item.right).to_str().unwrap().to_string()));
    }
    download.push((work.picture_url.clone(), path.join("picture.jpg").to_str().unwrap().to_string()));
    download.push((work.title_picture_url.clone(), path.join("title_picture.jpg").to_str().unwrap().to_string()));

    let mut full_url = String::from(&work.base_url);
    full_url.push_str(&work.model.file_url);
    download.push((full_url, path.join(&work.model.file_url).to_str().unwrap().to_string()));
    for item in work.model.material_textures.iter() {
        let mut full_url = String::from(&work.base_url);
        full_url.push_str(&work.model.material_base_url);
        full_url.push_str(&item);
        download.push((full_url, path.join(&work.model.material_base_url).join(&item).to_str().unwrap().to_string()));
    }

    for item in download.iter() {
        println!("{:?}", item);
        _ = do_download(item.0.clone(), item.1.clone()).await;
    }
}

async fn  do_download(url : String, dest : String) -> Result<(), String> {
    //let resp = reqwest::blocking::get(url);
    let client = reqwest::Client::new();
    let builder = client.get(url);
    let result = builder.send().await;

    if let Err(err) = result {
        return Err(err.to_string());
    }
    let response = result.unwrap();
    let path : &Path = Path::new(&dest);
    if let Err(err) = std::fs::create_dir_all(path.parent().unwrap()) {
        return Err(err.to_string())
    }
    println!("{}", dest);
    let res = File::create(dest);
    if res.is_err() {
        return Ok(());
    }
    let mut buffer = res.unwrap();
    //buffer.a
    let bytes  = response.bytes().await;
    if let Err(err) = buffer.write_all(&bytes.unwrap().to_vec()) {
        return Err(err.to_string());
    }
    //resp.bytes()
    Ok(())
}
