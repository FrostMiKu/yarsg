use std::path::Path;
use serde_derive::{Serialize, Deserialize};
use toml::value::Array;
use crate::errors::Result;
use log::error;

// themes
//   └─ThemeName
//       ├─assets
//       ├─layouts
//      ├─theme.toml
//       └─settings.toml

/// theme.toml
#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    description: String,
    homepage: String,
    license: Option<String>,
    min_version: String,
    tags: Option<Array>,
    author: Author,
}

impl Theme {
    pub fn parse(s: &str) -> Result<Self> {
        match toml::from_str(s) {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("The theme parse failed!");
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

#[derive(Serialize, Deserialize)]
struct Author {
    author: String,
    homepage: String,
}

/// settings.toml
#[derive(Serialize, Deserialize)]
struct Settings {

}