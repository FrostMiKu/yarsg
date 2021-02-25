use std::path::PathBuf;
use yarsg::config::Config;
use yarsg::config::site::Site;

#[test]
fn test_config_save(){
    let test_config:Site = Config::new();
    test_config.save(PathBuf::from("tmp/test.conf.toml").as_path());
}

#[test]
fn test_config_load(){
    let test_config:Site = Config::load(PathBuf::from("tmp/test.conf.toml").as_path());
}