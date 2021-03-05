use std::path::Path;
use crate::utils;
use std::result::Result::Err;
use serde::Serialize;
use log::error;
use crate::errors::Result;

pub mod site;
pub mod theme;

pub fn save<T: Serialize>(config: &T, path: &Path) -> Result<()>{
    let s = match toml::to_string(config) {
        Ok(s) => s,
        Err(e) => {
            error!("Convert the config to string failed!");
            return Err(e.into());
        }
    };
    utils::write_to_file(&s, path)?;
    return Ok(());
}