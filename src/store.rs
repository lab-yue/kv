extern crate serde;
extern crate serde_json;
extern crate dirs;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;

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
    match dirs::home_dir() {
        Some(mut home_path) => {
            
            home_path.push(".data/kv-data.json");

            #[cfg(debug_assertions)]
            println!("file path, probably: {}", home_path.display());

            let json_file = File::open(home_path).expect("file not found");
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
