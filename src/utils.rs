use std::env;
use std::io;
use std::path::{PathBuf, Path};
use std::fs;
use log::error;
use std::io::Write;

pub fn get_exe_dir_path() -> io::Result<PathBuf> {
    let mut dir = env::current_exe()?;
    dir.pop();
    Ok(dir)
}

pub fn write_to_file(s: &str, path: &Path) -> io::Result<()> {
    let mut f = match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path) {
        Ok(f) => f,
        Err(e) => {
            error!("Write to file failed! {}",e);
            return Err(e);
        },
    };
    f.write_all(s.as_bytes())
}