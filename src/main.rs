mod cpu_info;
mod system_info;

use clap::{App, Arg};
use sysinfo::{System, SystemExt};

fn main() {
    let matches = App::new("Linux Task Manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Displays system information")
        .arg(
            Arg::with_name("cpu")
            ""    .short("c")
                .long("cpu")
                .help("Displays CPU information"),
        )
        .arg(
            Arg::with_name("system")
                .short("s")
                .long("system")
                .help("Displays system information"),
        )
        .get_matches();

    let mut system = System::new_all();
    system.refresh_all();

    if matches.is_present("cpu") {
        cpu_info::print_cpu_info(&system);
    }

    if matches.is_present("system") {
        system_info::print_system_info(&system);
    }
}
