use std::{fs, path::PathBuf};
use super::config::{self, Config};
use log::{info, error};

pub fn init_site_workspace(root_dir: &PathBuf, site_name: &str){
    let work_dir = match root_dir.join(site_name).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            error!("Can't init site: {}", e);
            return;
        }
    };

    match fs::create_dir(work_dir.as_path()) {
        Ok(_) => (),
        Err(_) => {
            error!("Can't init site, dir exists: {}", work_dir.display());
            return;
        }
    };
    fs::create_dir(work_dir.join("themes").as_path()).unwrap();

    let site_config: config::site::Site = Config::new();
    site_config.save(&work_dir.join("config.toml"));
    info!("...Done.");
}