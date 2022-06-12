//! Everything related to logging

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
