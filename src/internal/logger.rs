//! logger

use env_logger::fmt::Color;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn initialize_logger(verbosity: LevelFilter) {
    env_logger::Builder::new()
        .filter_level(verbosity)
        .format(|buf, record| {
            let ts = buf.timestamp();
            let src = record.file().unwrap();
            let line = record.line().unwrap();
            let msg = record.args();

            let mut level_style = buf.style();
            let level = match record.level() {
                Level::Error => {
                    level_style.set_color(Color::Red).set_bold(true);
                    level_style.value(record.level())
                }
                Level::Info => {
                    level_style.set_color(Color::Green).set_bold(true);
                    level_style.value(record.level())
                }
                _ => {
                    level_style.set_color(Color::Yellow).set_bold(true);
                    level_style.value(record.level())
                }
            };

            writeln!(buf, "{} [{}] {}:{} | {}", ts, level, src, line, msg)
        })
        .init();
}
