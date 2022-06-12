#[macro_use]
extern crate log;

#[macro_use]
extern crate penrose;

/// Module containing functions to get key and mouse bindings to be used by
/// [penrose::WindowManager::grab_keys_and_run] method.
pub mod bindings {
    use penrose::{
        __test_helpers::{KeyBindings, MouseBindings},
        core::{helpers::index_selectors, xconnection::XConn},
        Backward, Forward, Less, More,
    };

    fn key<X: XConn>() -> KeyBindings<X> {
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
            "M-Return" => run_external!("alacritty");
            "M-A-Escape" => run_internal!(exit);

            map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
                "M-{}" => focus_workspace (REF);
                "M-S-{}" => client_to_workspace (REF);
            };
        }
    }

    fn mouse<X: XConn>() -> MouseBindings<X> {
        map! {}
    }
}

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
