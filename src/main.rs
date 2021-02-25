use log::{info,error};
use pulldown_cmark::{html, Options, Parser};
use std::{env, fs, path::PathBuf};
use yarsg::{logger, cli::build_cli};
use yarsg::config::{self, Config};

fn main() {
    logger::init();

    let matches = build_cli().get_matches();
    
    let root_dir = match matches.value_of("root").unwrap() {
        "." => env::current_dir().unwrap(),
        path => PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Cannot find root directory: {}", path)),
    };
    info!("root directory: {}",root_dir.display());

    match matches.subcommand() {
        ("new", Some(matches)) => {
            init_site_workspace(root_dir, matches.value_of("name").unwrap());
        }
        ("build", Some(matches)) => {

        }
        _ => unreachable!(),
    }

}

fn init_site_workspace(root_dir: PathBuf, site_name: &str){
    let work_dir = match root_dir.join(site_name).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            error!("Can't init site: {}",e);
            return;
        }
    };

    match fs::create_dir(work_dir.as_path()) {
        Ok(_) => (),
        Err(e) => {
            error!("Can't init site: {}",e);
            return;
        }
    };
    fs::create_dir(work_dir.join("themes").as_path()).unwrap();

    let site_config: config::site::Site = Config::new();
    site_config.save(&work_dir.join("config.toml"));
    info!("...Done.");
}