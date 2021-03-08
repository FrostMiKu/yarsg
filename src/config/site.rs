use std::path::Path;
use log::error;
use serde_derive::{Serialize, Deserialize};
use crate::errors::Result;

// config.toml
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // the version of the config file
    version: String,
    site: SiteConfig,
    features: FeaturesConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            version: "0.1".to_string(),
            site: SiteConfig::default(),
            features: FeaturesConfig::default(),
        }
    }
}

impl Config {
    fn parse(s: &str) -> Result<Self> {
        match toml::from_str(s) {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("The config parse failed!");
                Err(e.into())
            }
        }
    }

    pub fn from_file(path: &Path) -> Result<Self>{
        let file_name = path.file_name().unwrap();
        let s = crate::utils::read_file_or_error(
            path,
    format!("No `{:?}` file found. Are you in the right directory?", file_name)
        )?;
        Self::parse(&s)
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

impl Default for SiteConfig {
    fn default() -> SiteConfig {
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
    rss: bool, // todo
}

impl Default for FeaturesConfig {
    fn default() -> FeaturesConfig {
        FeaturesConfig {
            rss: false,
        }
    }
}
