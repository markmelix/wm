#[macro_use]
extern crate log;

use std::process;
use wm::logger;

fn main() {
	match logger::init() {
		Ok(_) => info!("Logger has been initialized successfully"),
		Err(e) => {
			eprintln!("Failed to initialize logger: {e}");
			process::exit(1);
		}
	}

	info!("Running window manager");

	if let Err(e) = wm::run() {
		error!("{e}");
		process::exit(1);
	}

	info!("Exited from the window manager successfully");
}
