#[macro_use]
extern crate penrose;

use result::Result;

/// Default terminal emulator
pub const TERMINAL: &str = "alacritty";

/// Run Window Manager.
pub fn run() -> Result<()> {
	use penrose::{logging_error_handler, xcb::new_xcb_backed_window_manager};

	let (key_bindings, mouse_bindings) = bindings::both();

	let config = config::get()?;
	let hooks = vec![];
	let error_handler = logging_error_handler();

	let mut wm = new_xcb_backed_window_manager(config, hooks, error_handler)?;

	Ok(wm.grab_keys_and_run(key_bindings, mouse_bindings)?)
}

/// Everything related to high-level configurations
pub mod config {
	use crate::{layouts, result::Result};
	use penrose::Config;

	/// Names of the workspaces
	pub const WORKSPACES: [&str; 9] =
		["term", "code", "web", "other", "5", "6", "7", "8", "9"];

	/// Classes of the floating windows
	pub const FLOATING_CLASSES: Vec<&str> = vec![];

	/// Color of focused window border
	pub const FOCUSED_BORDER_COLOR: &str = "#FFFFFF";

	/// Color of unfocused window border
	pub const UNFOCUSED_BORDER_COLOR: &str = "#000000";

	/// Border width in pixels
	pub const BORDER_WIDTH: u32 = 2;

	/// Size of gap between windows in pixels
	pub const GAP_SIZE: u32 = 5;

	/// The percentage of the screen to grow the main region by when incrementing
	pub const MAIN_RATIO_STEP: f32 = 0.1;

	/// Whether or not space should be reserved for a status bar
	pub const SHOW_BAR: bool = true;

	/// Whether or not the reserved space for a status bar is at the top of the sceen
	pub const TOP_BAR: bool = true;

	/// the height of the space to be reserved for a status bar in pixels
	pub const BAR_HEIGHT: u32 = 15;

	/// Get [config][penrose::Config]
	pub fn get() -> Result<Config> {
		let layouts = layouts::get();
		let config = Config::default()
			.builder()
			.workspaces(WORKSPACES)
			.floating_classes(FLOATING_CLASSES)
			.layouts(layouts)
			.focused_border(FOCUSED_BORDER_COLOR)?
			.unfocused_border(UNFOCUSED_BORDER_COLOR)?
			.border_px(BORDER_WIDTH)
			.gap_px(GAP_SIZE)
			.main_ratio_step(MAIN_RATIO_STEP)
			.show_bar(SHOW_BAR)
			.top_bar(TOP_BAR)
			.bar_height(BAR_HEIGHT)
			.build()?;

		Ok(config)
	}
}

/// Everything related to layouts
pub mod layouts {
	use penrose::core::layout::{Layout, LayoutFunc};

	const MAX_MAIN: u32 = 1;
	const MAIN_RATIO: f32 = 0.5;

	/// Get used layout functions
	pub fn get() -> Vec<penrose::core::Layout> {
		use penrose::core::layout::{bottom_stack, monocle, side_stack};

		vec![
			layout("[mono]", monocle),
			layout("[side]", side_stack),
			layout("[bottom]", bottom_stack),
		]
	}

	fn layout(symbol: impl Into<String>, f: LayoutFunc) -> Layout {
		Layout::new(symbol, Default::default(), f, MAX_MAIN, MAIN_RATIO)
	}
}

/// Module containing functions to get key and mouse bindings to be used by
/// [`grab_keys_and_run`][penrose::WindowManager::grab_keys_and_run] method.
pub mod bindings {
	use penrose::{
		__test_helpers::{KeyBindings, MouseBindings},
		core::{helpers::index_selectors, xconnection::XConn},
		Backward, Forward, Less, More,
	};

	/// Get key bindings
	pub fn key<X: XConn>() -> KeyBindings<X> {
		use crate::TERMINAL;

		gen_keybindings! {
			"M-j" => run_internal!(cycle_client, Forward);
			"M-k" => run_internal!(cycle_client, Backward);
			"M-S-j" => run_internal!(drag_client, Forward);
			"M-S-k" => run_internal!(drag_client, Backward);
			"M-S-q" => run_internal!(kill_client);
			"M-Tab" => run_internal!(toggle_workspace);
			"M-grave" => run_internal!(cycle_layout, Forward);
			"M-S-grave" => run_internal!(cycle_layout, Backward);
			"M-A-Up" => run_internal!(update_max_main, More);
			"M-A-Down" => run_internal!(update_max_main, Less);
			"M-A-Right" => run_internal!(update_main_ratio, More);
			"M-A-Left" => run_internal!(update_main_ratio, Less);
			"M-semicolon" => run_external!("dmenu_run");
			"M-Return" => run_external!(TERMINAL);
			"M-A-Escape" => run_internal!(exit);

			map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
				"M-{}" => focus_workspace (REF);
				"M-S-{}" => client_to_workspace (REF);
			};
		}
	}

	/// Get mouse bindings
	pub fn mouse<X: XConn>() -> MouseBindings<X> {
		map! {}
	}

	/// Get both key and mouse bindings
	pub fn both<X: XConn>() -> (KeyBindings<X>, MouseBindings<X>) {
		return (key(), mouse());
	}
}

// Everything related to logging
pub mod logger {
	use log::LevelFilter;
	use simplelog::{ColorChoice, TermLogger};

	use crate::result::Result;

	/// Initialize simple logger
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
