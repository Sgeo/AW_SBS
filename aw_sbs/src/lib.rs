extern crate easyhook;
extern crate libloading as lib;

#[macro_use]
extern crate lazy_static;

use std::os::raw::{c_ulong, c_void};
use easyhook::lh_install_hook;

lazy_static! {
    static ref GL: lib::Library = lib::Library::new("OPENGL32").unwrap();
    static ref glViewport: lib::Symbol<'static, extern "system" fn(i32, i32, u32, u32)> = unsafe { GL.get(b"glViewport\0") }.unwrap();
}

#[no_mangle]
pub extern "system" fn NativeInjectionEntryPoint(_remote_info: *mut c_void) {
    unsafe {
        lh_install_hook(**glViewport as *mut _, glViewportHook as *mut _);
    }
}

pub extern "system" fn glViewportHook(x: i32, y: i32, width: u32, height: u32) {
    glViewport(x, y, width/2, height);
}