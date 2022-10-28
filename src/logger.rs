use ansi_term::Color::{Cyan, Red, Yellow};
use eyre::Result;
use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};

pub struct Logger;

impl Logger {
    pub fn init() -> Result<()> {
        set_logger(&Logger)
            .map(|_| set_max_level(LevelFilter::Info))
            .map_err(|e| eyre::eyre!("failed to set logger: ${e:?}"))
    }
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Info => println!("{} {}", Cyan.bold().paint("[i]"), record.args()),
            Level::Warn => eprintln!("{} {}", Yellow.bold().paint("[w]"), record.args()),
            Level::Error => eprintln!("{} {}", Red.bold().paint("[e]"), record.args()),
            _ => {}
        }
    }

    fn flush(&self) {}
}
