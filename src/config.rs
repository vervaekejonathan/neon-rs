// Documentation
// https://codingpackets.com/blog/rust-load-a-toml-file/

use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
struct Data {
    states: States,
}

#[derive(Deserialize)]
struct States {
    list : Vec<List>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct List {
    pub name: String,
    pub value: String
}

pub fn decode() -> Vec<List> {
    // Variable that holds the filename as a `&str`.
    let filename = "./config/config.toml";
    let contents = match fs::read_to_string(filename) {

        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    data.states.list.iter().for_each(|l| {
        println!("{:?}", l);
    });

    data.states.list
}