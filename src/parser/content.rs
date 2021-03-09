use lazy_static::lazy_static;
use regex::Regex;
use crate::config::metadata::Metadata;
use crate::errors::{Result, Error};

lazy_static! {
    static ref METADATA_RE: Regex = Regex::new(
        r"^[[:space:]]*\+\+\+(\r?\n(?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$"
    ).unwrap();
}

pub struct Content {
    metadata: Metadata,
    markdown: String,
}

impl Content {
    fn parse(s: &str) -> Result<Self> {
        if !METADATA_RE.is_match(s) {
            return Err(Error::new("match metadata failed!"));
        }
        let caps = METADATA_RE.captures(s).unwrap();

        let content = Content {
            metadata: match Metadata::parse(caps.get(1).unwrap().as_str()) {
                Ok(m) => m,
                Err(e) => return Err(e),
            },
            markdown: caps.get(2).unwrap().as_str().to_string(),
        };

        if content.metadata.id.is_none() {
            return Err(Error::new("miss metadata 'id', consider run 'yarsg fix' to fix it."));
        }

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex () {
        let s = r#"
+++
type = "post"
id = 1
+++
Yet Another Rust Site Generator
        "#;
        assert!(METADATA_RE.is_match(s));

        let caps = METADATA_RE.captures(s).unwrap();
        for i in caps.iter() {
            i.map(|s|{
                println!("{}\n------",s.as_str());
            });
        }
    }
}