use std::os::raw::{c_ulong, c_void};
use std::ptr;
use std::ffi::OsStr;

use widestring::WideCString;

#[repr(C)]
pub struct HOOK_TRACE_INFO {
    link: *mut c_void
}

impl HOOK_TRACE_INFO {
    pub fn new() -> Self {
        HOOK_TRACE_INFO {
            link: ptr::null_mut()
        }
    }
}

#[link(name="easyhook32")]
extern "system" {
    pub fn RhInjectLibrary(pid: c_ulong, wakeup_thread: c_ulong, options: c_ulong, library_x86: *mut u16, library_x64: *mut u16, passthrough: *mut c_void, passthrough_size: usize);
    pub fn LhInstallHook(entry: *mut c_void, hook: *mut c_void, callback: *mut c_void, trace_info: *mut HOOK_TRACE_INFO);
}

