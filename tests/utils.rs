use std::{env, path::PathBuf};
use yarsg::utils;
#[test]
fn test_get_exe_dir_path() {
    let path = utils::get_exe_dir_path().unwrap();
    let mut exp = env::current_dir().unwrap().join("target");
    exp.push("debug");
    exp.push("deps");

    assert_eq!(path,exp)
}

#[test]
fn test_read_file_or_error() {
    let path = PathBuf::from("src/main.rs");
    utils::read_file_or_error(path.as_path(), "foo".to_string()).unwrap();
}