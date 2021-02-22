use std::io::Write;
use chrono::Local;
use log::LevelFilter;
use env_logger::{Builder, Target, fmt::Color};

pub fn init() {
 
    let mut builder = Builder::new();
    builder.target(Target::Stdout).filter(None, LevelFilter::Debug);

    builder.format(|buf, record| {

        let color = match record.level() {
            log::Level::Info => Color::Green,
            log::Level::Warn => Color::Yellow,
            log::Level::Error => Color::Red,
            log::Level::Debug => Color::Magenta,
            _ => Color::Red,
        };
        let mut style = buf.style();
        style.set_color(color);

        writeln!(buf,
            "{} [{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            style.value(record.level()),
            style.value(record.args()),
        )
    });
 
    builder.init();
}