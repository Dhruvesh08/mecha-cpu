use cpu::{MyCPUMonitor, CPUMonitor};

mod cpu;

fn main() {
    let monitor = MyCPUMonitor;

    let zone = 0; // Specify the thermal zone (0 or 1) for the CPU temperature you want to read
    match monitor.cpu_temp(zone) {
        Ok(temp) => println!("CPU temperature (zone {}): {:.2}°C", zone, temp),
        Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
    }

    let cpu_max_prime = 90000;
    print!("Running stress test... ");
    match monitor.stress_test_cpu(cpu_max_prime) {
        Ok(()) => println!("Command executed successfully!"),
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }

    match monitor.cpu_temp(zone) {
        Ok(temp) => println!("CPU temperature (zone {}): {:.2}°C", zone, temp),
        Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
    }
}