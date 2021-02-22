use std::fs;
use pulldown_cmark::{Parser, Options, html};
use log::{info, warn, error, debug};
use yarsg::logger;

fn main() {
    logger::init();

    error!("error message");
    warn!("warn message");
    info!("info message");
    debug!("debug message");
}