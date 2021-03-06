use log::error;
use serde_derive::{Deserialize, Serialize};
use toml::value::{Array, Datetime};
use crate::errors::Result;

// metadata of post
#[derive(Deserialize, Serialize)]
pub struct Metadata {
    id: Option<u64>,
    // theme file, default:post
    r#type: Option<String>,
    // default: the value of the keyword 'owner' in the config.toml
    authors: Option<Array>,
    tags: Option<Array>,
    // default: now
    create_date: Option<Datetime>,
    modify_date: Option<Datetime>,
}

impl Metadata {
    pub fn parse(s: &str) -> Result<Self> {
        match toml::from_str(s) {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("The metadata parse failed!");
                Err(e.into())
            }
        }
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
    }
}