use yarsg::logger;
use log::{debug, error, info, warn};

#[test]
fn test_logger() {
    logger::init();

    debug!("debug message");
    error!("error message");
    warn!("warn message");
    info!("info message");
}
