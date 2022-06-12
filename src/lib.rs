#[macro_use]
extern crate log;

// Everything related to logging
pub mod logger {
    use log::LevelFilter;
    use simplelog::{ColorChoice, TermLogger};

    use crate::result::Result;

    /// Initialize simple logger.
    pub fn init() -> Result<()> {
        Ok(TermLogger::init(
            LevelFilter::Trace,
            Default::default(),
            Default::default(),
            ColorChoice::Auto,
        )?)
    }
}

pub mod result {
    use std::{error, result};

    pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
}
