use sysinfo::{System, SystemExt};

pub fn print_system_info(system: &System) {
    println!("System name: {:?}", system.name());
    println!("System kernel version: {:?}", system.kernel_version());
    println!("System OS version: {:?}", system.os_version());
    println!("System host name: {:?}", system.host_name());
}