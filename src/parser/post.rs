use std::path::PathBuf;
use crate::config::metadata::Metadata;
use crate::errors::Result;


pub struct Post {
    filepath: PathBuf,
    metadate: Metadata,
    markdown: String,
}

impl Post {
    fn parse(s: &str) -> Result<Self> {
        todo!()
    }
}
