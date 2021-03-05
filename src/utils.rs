use std::env;
use std::path::{PathBuf, Path};
use std::fs;
use std::io::Write;
use crate::errors::{Error, Result};

pub fn get_exe_dir_path() -> Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

pub fn write_to_file(s: &str, path: &Path) -> Result<()> {
    let mut f = match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(Error::new(format!("Write to file failed! {}",e)));
        },
    };
    f.write_all(s.as_bytes())?;
    Ok(())
}

pub fn read_file_or_error(path: &Path, err_msg: String) -> Result<String>{
    match fs::read_to_string(path){
        Ok(s) => Ok(s),
        Err(_) => {
            Err(Error::new(err_msg))
        }
    }
}