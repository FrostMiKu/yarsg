use std::{fs, path::{Path, PathBuf}};
use crate::errors::Result;
use crate::config::site;


pub fn build_site(root_dir: &PathBuf, site_name: &str) -> Result<()> {
    let config = match site::Config::from_file(
        root_dir.join(site_name).join("config.toml").as_path()
    ) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.into());
        },
    };
    todo!()
}

fn read_md_from_dir(path: &Path) {
    let paths = fs::read_dir(path).unwrap();

    for p in paths {
        println!("Name: {}", p.unwrap().path().display())
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn test_read_md_from_dir() {
        let path = PathBuf::from_str("src").unwrap();
        read_md_from_dir(path.as_path());
    }
}