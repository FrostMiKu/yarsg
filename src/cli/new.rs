use std::{fs, path::PathBuf};
use super::config;
use log::info;
use crate::errors::{Error, Result};

pub fn init_site_workspace(root_dir: &PathBuf, site_name: &str) -> Result<()>{

    let work_dir = root_dir.join(site_name);

    if let Err(_) = fs::create_dir(work_dir.as_path()) {
        return Err(Error::new(
        format!("Can't init site, dir exists: {}", work_dir.display())
        ));
    }


    let site_config = config::site::Config::default();
    if let Err(e) = config::save(&site_config, &work_dir.join("config.toml")) {

        // init failed! clean dir...
        fs::remove_dir_all(work_dir.as_path()).unwrap();
        return Err(e);
    }

    // we can sure that never panic
    fs::create_dir(work_dir.join("themes").as_path()).unwrap();
    
    info!("...Done.");
    Ok(())
}