use std::{fmt::Display, sync::OnceLock};

use clap::ValueEnum;

static LOG_SINGLETON: OnceLock<Logger> = OnceLock::new();

#[derive(Default)]
pub struct Logger {
    min_lvl: Verbosity,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum Verbosity {
    //only the response body
    Minimal,
    //response headers, body and time it took to finish
    #[default]
    Normal,
    //parsed request and all info from the other levels.
    Detailed,
}

pub fn init(level: Verbosity) -> &'static Logger {
    LOG_SINGLETON.get_or_init(|| Logger { min_lvl: level })
}

fn get_instance() -> Option<&'static Logger> {
    LOG_SINGLETON.get()
}

pub fn log(message: &impl Display, level: Verbosity) {
    if let Some(logger) = get_instance() {
        logger.log(message, level);
    }
}

pub fn log_msg(message: &str, level: Verbosity) {
    if let Some(logger) = get_instance() {
        logger.log_msg(message, level);
    }
}

impl Logger {
    fn log(&self, message: &impl Display, level: Verbosity) {
        if self.min_lvl >= level {
            println!("{message}");
        }
    }

    fn log_msg(&self, msg: &str, level: Verbosity) {
        if self.min_lvl >= level {
            println!("{msg}");
        }
    }
}
