use sysinfo::{System, SystemExt, CpuExt};

pub fn print_cpu_info(system: &System) {
    println!("CPU Cores: {}", system.cpus().len());
    for (i, cpu) in system.cpus().iter().enumerate() {
        println!("CPU {}: {}%", i, cpu.cpu_usage());
    }
}

