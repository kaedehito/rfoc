#![deny(warnings)]
use crate::result_trait::{RfocOptionExtended, RfocResultExtended};
use dirs_next;
use serde::{Deserialize, Serialize};
use std::{
    fs, io::Write, path::Path
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub syntax: Syntax,
    pub interactive: Interactive,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Syntax {
    pub enable: bool,
    pub theme: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Interactive{
    pub new_buffer: bool,
}

pub fn get_config() -> Config {
    let path = dirs_next::config_dir().rfoc_unwrap();

    let rfoc = format!("{}/rfoc/", path.display());
    let foc: &Path = rfoc.as_ref();

    if !foc.exists() {
        println!("[INFO] Creating {rfoc}...");
        fs::create_dir(foc).rfoc_unwrap();
    }

    let s = format!("{}/rfoc/config.toml", path.display());
    let sp: &Path = s.as_ref();

    if !sp.exists() {
        println!("[INFO] Generating config file...");
        let conf = include_bytes!("../config_example.toml");
        let mut file = fs::File::create(sp).rfoc_unwrap();
        file.write_all(conf).rfoc_unwrap();
    }

    let config = fs::read_to_string(sp).rfoc_unwrap();
    let config: Config = toml::from_str(&config).rfoc_unwrap();
    config
}
