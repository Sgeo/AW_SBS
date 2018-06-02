extern crate widestring;

mod sys;

use std::os::raw::{c_ulong, c_void};
use std::ptr;
use std::ffi::OsStr;
use std::mem;

use widestring::WideCString;


pub fn rh_inject_library<S: AsRef<OsStr>>(pid: usize, library: S) {
    unsafe {
        sys::RhInjectLibrary(pid, 0, 0, WideCString::from_str(library).unwrap().into_raw(), ptr::null_mut(), ptr::null_mut(), 0);
    }
}

pub unsafe fn lh_install_hook(entry: *mut c_void, hook: *mut c_void) {
    let mut hook_trace_info = Box::new(sys::HOOK_TRACE_INFO::new());
    sys::LhInstallHook(entry, hook, ptr::null_mut(), &mut *hook_trace_info as *mut _);
    mem::forget(hook_trace_info);
}

pub fn error_string() -> Option<String> {
    let err = unsafe {sys::RtlGetLastErrorString()};
    if(!err.is_null()) {
        Some(unsafe { WideCString::from_ptr_str(err) }.to_string_lossy())
    } else {
        None
    }
}