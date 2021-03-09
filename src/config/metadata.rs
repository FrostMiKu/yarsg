use std::str::FromStr;

use chrono::Local;
use log::error;
use serde_derive::{Deserialize, Serialize};
use toml::value::{Array, Datetime};
use crate::errors::Result;

/// Metadata of the content.
#[derive(Deserialize, Serialize)]
pub struct Metadata {
    /// Must be set.
    pub id: Option<u64>,
    /// Template filename. content will be skipped if it is 'None'.
    pub r#type: Option<String>,
    /// If it is 'None', it will be set to the filename.
    pub title: Option<String>,
    /// If it is 'None', it will be set to 
    /// the value of the keyword 'owner' in the config.toml.
    pub authors: Option<Array>,
    pub tags: Option<Array>,
    /// Default: now.
    pub create_date: Option<Datetime>,
    pub modify_date: Option<Datetime>,
}

impl Metadata {
    pub fn parse(s: &str) -> Result<Self> {
        let mut c: Self = match toml::from_str(s) {
            Ok(c) => c,
            Err(e) => {
                error!("The metadata parse failed!");
                return Err(e.into());
            }
        };

        // default value of the create_date
        if let None = c.create_date {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            c.create_date = Some(Datetime::from_str(&time).unwrap());
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = r#"          
        type = "post"         
        authors = ["lethon",]
        "#;

        let metadata = Metadata::parse(s).unwrap();
        let str = toml::to_string(&metadata).unwrap();
        println!("{}",str);

        assert_eq!(metadata.r#type, Some("post".to_string()));
        assert_eq!(metadata.id, None);

        let author = &metadata.authors.unwrap()[0];
        assert_eq!(author.as_str().unwrap(), "lethon");

        println!("{}", metadata.create_date.unwrap());
    }
}