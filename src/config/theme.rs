use serde_derive::{Serialize, Deserialize};
use toml::value::Array;

// theme.toml
#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    description: String,
    url: String,
    license: Option<String>,
    min_version: String,
    tags: Option<Array>,
    author: Author,
}

impl Theme {
    
}

#[derive(Serialize, Deserialize)]
struct Author {
    author: String,
    homepage: String,
}

// settings.toml
#[derive(Serialize, Deserialize)]
struct Settings {

}