use std::{fs, path::Path};
use super::utils;
use std::result::Result::Err;
use serde::{Deserialize, Serialize};
use log::error;

pub mod site;
pub mod theme;

// need refactor

pub fn save<T: Serialize>(config: &T, path: &Path) {
    let s = match toml::to_string(config) {
        Ok(s) => s,
        Err(e) => {
            error!("The config file save failed!");
            panic!(e);
        }
    };
    utils::write_to_file(&s, path).unwrap();
}

// pub fn load<'a, T>(path: &Path) -> T
//     where T: Deserialize<'a> {
//     let s = match fs::read_to_string(path){
//         Ok(s) => s,
//         Err(e) => {
//             error!("The config file load failed!");
//             panic!(e);
//         }
//     };

//     match toml::from_str(&s) {
//         Ok(c) => c,
//         Err(e) => {
//             error!("The config file load failed!");
//             panic!(e);
//         }
//     }
// }