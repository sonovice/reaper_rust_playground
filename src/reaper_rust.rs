include!("reaper_bindings.rs");

extern crate user32;

extern crate libc;

use libc::{c_int, c_void, c_char};

#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::collections::HashMap;
use std::sync::Mutex;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::{EnumProperty, IntoEnumIterator, AsStaticRef};

lazy_static! {
    static ref ACTIONS_MAP: Mutex<HashMap<c_int, Action>> = Mutex::new(HashMap::new());
}

fn register_action(
    register_func: unsafe extern "C" fn(*const c_char, *mut libc::c_void) -> c_int,
    action: Action,
    fVirt: BYTE,
    key: WORD,
) -> c_int {
    let c_name = CString::new("command_id").unwrap();
    let c_infostruct = CString::new(action.as_static()).unwrap();
    let command_id: c_int = unsafe { register_func(c_name.as_ptr() as *const c_char, c_infostruct.as_ptr() as *mut c_void) };

    let accel = ACCEL { fVirt, key, cmd: command_id as WORD };
    let c_description = CString::new(action.get_str("Desc").unwrap()).unwrap();
    let mut accel_reg = gaccel_register_t {
        accel,
        desc: c_description.into_raw() as *const c_char,
    };

    let c_name = CString::new("gaccel").unwrap();
    unsafe { register_func(c_name.as_ptr() as *const c_char, &mut accel_reg as *mut _ as *mut c_void) };

    let c_name = CString::new("hookcommand").unwrap();
    unsafe { register_func(c_name.as_ptr() as *const c_char, _actions_callback as *mut c_void) };

    ACTIONS_MAP.lock().unwrap().insert(command_id, action);

    command_id
}

fn _actions_callback(command_id: c_int, _flag: c_int) -> c_int {
    if ACTIONS_MAP.lock().unwrap().contains_key(&command_id) {
        let lock = ACTIONS_MAP.lock().unwrap();
        let action = lock.get(&command_id).unwrap();
        actions_callback(action, _flag);
        return 1; // Successfully handled
    }
    0 // Action not handled by this plugin
}

#[no_mangle]
pub extern "C" fn ReaperPluginEntry(_instance: HINSTANCE, rec: *mut reaper_plugin_info_t) -> c_int {
    if rec.is_null() {
        on_plugin_unload();
        return 0;
    }

    // Exit due to incompatible plugin version
    if unsafe { *rec }.caller_version != REAPER_PLUGIN_VERSION as i32 {
        return 0;
    }

    // Success. go ahead and init stuff...
    for cmd in Action::iter() {
        register_action(unsafe { *rec }.Register.unwrap(), cmd, 0, 0);
    }
    on_plugin_load();

    1 // Return "1" to denote success
}

fn show_message(message: String) {
    let text = CString::new(message).unwrap();
    let title = CString::new("Reaper Rust Example").unwrap();
    unsafe {
        MessageBoxA(::std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), MB_OK | MB_ICONINFORMATION);
    }
}