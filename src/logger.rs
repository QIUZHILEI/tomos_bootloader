use core::ptr::addr_of;

use crate::println;
use log::{Level, Metadata, Record};
const DEFAULT_LEVEL: Level = Level::Trace;
static mut LOGGER: SimpleLogger = SimpleLogger::new();

pub fn init(level: Level) {
    let log_ref = unsafe {
        LOGGER.level = level;
        addr_of!(LOGGER).as_ref().unwrap()
    };
    log::set_logger(log_ref)
        .map(|_| log::set_max_level(level.to_level_filter()))
        .unwrap();
}

struct SimpleLogger {
    level: Level,
}

impl SimpleLogger {
    const fn new() -> Self {
        Self {
            level: DEFAULT_LEVEL,
        }
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "[{}] - [{}] - {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
