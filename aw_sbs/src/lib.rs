#![crate_type="cdylib"]

extern crate easyhook;
extern crate libloading as lib;

#[macro_use]
extern crate lazy_static;

use std::io::Write;
use std::os::raw::{c_ulong, c_void};
use easyhook::lh_install_hook;
use easyhook::error_string;

lazy_static! {
    static ref GL: lib::Library = lib::Library::new("OPENGL32").unwrap();
    static ref glViewport: lib::Symbol<'static, extern "system" fn(i32, i32, u32, u32)> = unsafe { GL.get(b"glViewport\0") }.unwrap();
}

#[export_name="_NativeInjectionEntryPoint_4"]
pub extern "stdcall" fn NativeInjectionEntryPoint(_remote_info: *mut c_void) {
    unsafe {
        use std::fs::File;
        File::create("about_to_install_hook.txt").unwrap();
        lh_install_hook(**glViewport as *mut _, glViewportHook as *mut _);
        let error = error_string();
        File::create("installed_hook.txt").unwrap();
        let mut errors = File::create("hook_errors.txt").unwrap();
        writeln!(&mut errors, "Error: {:?}", error);
        drop(errors);
    }
}

pub extern "system" fn glViewportHook(x: i32, y: i32, width: u32, height: u32) {
    glViewport(x, y, width/2, height);
}