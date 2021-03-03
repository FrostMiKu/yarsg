use serde_derive::{Serialize, Deserialize};

// config.toml
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    site: SiteConfig,
    features: FeaturesConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            site: SiteConfig::default(),
            features: FeaturesConfig::default(),
        }
    }

    fn parse(s: &str) -> Config {
        match toml::from_str(s) {
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
    katex: bool, // todo
}

impl Default for FeaturesConfig {
    fn default() -> FeaturesConfig {
        FeaturesConfig {
            katex: false,
        }
    }
}
