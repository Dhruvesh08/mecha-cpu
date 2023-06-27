use cpu::cpu_temp;

mod cpu;
fn main() {
    let zone = 0; // Specify the thermal zone (0 or 1) for the CPU temperature you want to read

    match cpu_temp(zone) {
        Ok(temp) => println!("CPU temperature (zone {}): {:.2}°C", zone, temp),
        Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
    }
}