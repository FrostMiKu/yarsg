use log::LevelFilter;
use env_logger::{Builder, Target};

pub fn init() {
    // let format = |buf:&mut Formatter,record: &Record| {
    //     writeln!(buf, "{}: {}", record.level(), record.args())
    // };
 
    let mut builder = Builder::new();
    builder.target(Target::Stdout).filter(None, LevelFilter::Debug);

    // builder.format(format);
 
    builder.init();
}