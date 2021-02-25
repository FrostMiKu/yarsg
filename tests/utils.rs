use std::env;
use yarsg::utils;
#[test]
fn test_get_exe_dir_path() {

    let path = utils::get_exe_dir_path().unwrap();
    let mut exp = env::current_dir().unwrap().join("target");
    exp.push("debug");
    exp.push("deps");

    assert_eq!(path,exp)
}