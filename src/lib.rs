#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
include!("reaper_rust.rs");

// Define all actions here
#[derive(EnumProperty, EnumIter, AsStaticStr)]
pub enum Action {
    #[strum(props(Desc = "My first Rust action"))]
    RustTestAction1,
    #[strum(props(Desc = "My second Rust action"))]
    RustTestAction2,
}

pub fn actions_callback(action: &Action, _flag: c_int) {
    match action {
        Action::RustTestAction1 => show_message(String::from("Yay! Action 1 works!")),
        Action::RustTestAction2 => show_message(String::from("Yay! Action 2 works!")),
    }
}

fn on_plugin_load() {}

fn on_plugin_unload() {}
