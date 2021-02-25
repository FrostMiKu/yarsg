use serde_derive::{Serialize, Deserialize};
use crate::config::Config;
use std::path::Path;
use std::fs;
use log::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    site: SiteConfig,
    features: FeaturesConfig,
}

impl Config for Site {
    fn new() -> Site {
        Site {
            site: SiteConfig::new(),
            features: FeaturesConfig::new(),
        }
    }

    fn load(path: &Path) -> Site {
        let config = match fs::read_to_string(path){
            Ok(s) => s,
            Err(e) => {
                error!("The config file load failed!");
                panic!(e);
            }
        };

        match toml::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                error!("The config file load failed!");
                panic!(e);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SiteConfig {
    url: String,
    title: String,
    owner: String,
    source_dir: String,
    output_dir: String,
    theme: String,
}

impl SiteConfig {
    fn new() -> SiteConfig {
        SiteConfig {
            url: "https://yourname.github.io".to_string(),
            title: "yars".to_string(),
            owner: "username".to_string(),
            source_dir: "/absolute/path/post".to_string(),
            output_dir: "/absolute/path/build".to_string(),
            theme: "Default".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FeaturesConfig {
    katex: bool, // todo
}

impl FeaturesConfig {
    fn new() -> FeaturesConfig {
        FeaturesConfig {
            katex: false,
        }
    }
}
