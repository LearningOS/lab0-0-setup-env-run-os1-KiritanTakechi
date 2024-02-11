use log::{Level, Log, Record};

use crate::println;

pub struct Logger;

impl Log for Logger{
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level2color = |level: Level| match level {
            Level::Error => "31",
            Level::Warn => "33",
            Level::Info => "32",
            Level::Debug => "34",
            Level::Trace => "35",
        };

        (self.enabled(record.metadata())).then(|| {
            let color = level2color(record.level());
    
            println!(
                "\u{1B}[{}m[{:^5}] {}\u{1B}[0m",
                color,
                record.level(),
                record.args(),
            );
        });
    }

    fn flush(&self) {}
}

