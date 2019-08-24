extern crate dirs;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pair {
    pub k: String,
    pub v: String,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.k, self.v)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    data: Vec<Pair>,
}

pub fn read() -> Vec<Pair> {
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
                file.write_all(b"{\"data\":[]}")
                    .expect("Failed to create file");
            }

            #[cfg(debug_assertions)]
            println!("file path, probably: {}", data_path.display());

            let json_file = fs::File::open(&data_path).expect("file not found");
            let store: Store =
                serde_json::from_reader(json_file).expect("error while reading json");

            store.data
        }
        None => {
            println!("Impossible to get your home dir!");
            std::process::exit(1);
        }
    }
}
