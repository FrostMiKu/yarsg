use std::path::PathBuf;
use yarsg::config;
use yarsg::config::site::Config;

#[test]
fn test_config_save(){
    let test_config = Config::default();
    config::save(&test_config, PathBuf::from("tmp/test.conf.toml").as_path()).unwrap();
}

// //#[test]
// fn test_config_load(){
//     let test_config:Site = Config::load(PathBuf::from("tmp/test.conf.toml").as_path());
// }