extern crate easyhook;
extern crate sysinfo;

use easyhook::rh_inject_library;

use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();
    system.refresh_processes();
    let aw = system.get_process_list().values().filter(|process| process.name().to_lowercase() == "aworld.exe").next();
    if let Some(awproc) = aw {
        println!("Found AW process id: {}", awproc.pid());
        rh_inject_library(awproc.pid(), "aw_sbs.dll");
    } else {
        println!("Unable to find AW!");
    }
}
