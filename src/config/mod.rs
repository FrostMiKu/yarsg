use std::path::Path;
use super::utils;
use std::result::Result::Err;
use serde::Serialize;
use log::error;

pub mod site;

// need refactor
pub trait Config {
    fn new() -> Self;

    fn save(&self, path: &Path)
        where Self: Serialize
    {
        let config = match toml::to_string(&self) {
            Ok(s) => s,
            Err(e) => {
                error!("The config file save failed!");
                panic!(e);
            }
        };
        utils::write_to_file(&config, path).unwrap();
    }

    fn load(path: &Path) -> Self;
}