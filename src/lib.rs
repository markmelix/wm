#[macro_use]
extern crate log;

pub mod logger;

pub mod result {
    use std::{error, result};

    pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
}
