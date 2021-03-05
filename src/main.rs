use log::{error, info};
use std::{env, path::PathBuf};
use yarsg::{cli::{self, build_cli}, logger};

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
            if let Err(e) = cli::new::init_site_workspace(
                &root_dir, 
       matches.value_of("name").unwrap()
            ) {
                error!("{}",e);
            }
        }
        ("build", Some(_matches)) => {

        }
        _ => unreachable!(),
    }
}

