#[macro_use]
extern crate log;

#[macro_use]
extern crate penrose;

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

/// Module containing functions to get key and mouse bindings to be used by
/// [penrose::WindowManager::grab_keys_and_run] method.
pub mod bindings {
    use penrose::new::{
        bindings::{KeyBindings, MouseBindings},
        xconnection::XConn,
    };

    fn key<X: XConn>() -> KeyBindings<X> {
        map! {}
    }

    fn mouse<X: XConn>() -> MouseBindings<X> {
        map! {}
    }
}
