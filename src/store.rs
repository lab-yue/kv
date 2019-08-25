extern crate dirs;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    path: PathBuf,
    pub data: HashMap<String, String>,
}

pub fn new() -> Store {
    let data_path = Store::get_path();
    let data = Store::read(&data_path);
    return Store {
        path: data_path,
        data: data,
    };
}

impl Store {
    fn get_path() -> PathBuf {
        match dirs::data_local_dir() {
            Some(mut data_path) => {
                data_path.push("kv");

                if !data_path.exists() {
                    #[cfg(debug_assertions)]
                    dbg!(&data_path);

                    fs::create_dir(&data_path).expect("failed to create data dir");
                }

                data_path.push("data.json");

                if !data_path.exists() {
                    let mut file = fs::File::create(&data_path).expect("Failed to create file");
                    file.write_all(b"{}").expect("Failed to create file");
                }

                #[cfg(debug_assertions)]
                println!("file path, probably: {}", data_path.display());

                data_path
            }
            None => {
                println!("Impossible to get your home dir!");
                std::process::exit(1);
            }
        }
    }

    pub fn read(path: &PathBuf) -> HashMap<String, String> {
        let json_file = fs::File::open(path).expect("file not found");
        let data: HashMap<String, String> =
            serde_json::from_reader(json_file).expect("error while reading json");
        data
    }

    pub fn save(&self) {
        let json_string = serde_json::to_string(&self.data).expect("Failed to serialize data");

        if self.path.exists() {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .expect("Failed to open file");
            file.write_all(&json_string.as_bytes())
                .expect("Failed to write file");
        }
    }

    pub fn get(&self, key: &str) {
        if let Some(value) = self.data.get(key) {
            #[cfg(debug_assertions)]
            dbg!(value);
            println!("{}", value);
            return;
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        self.save();
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.save();
    }
}
