use std::fs;

fn cpu_temp(zone: u8) -> Result<f64, std::io::Error> {
    let zone_path = format!("/sys/class/thermal/thermal_zone{}/temp", zone);
    let temp_string = fs::read_to_string(zone_path)?;
    let temp: f64 = temp_string.trim().parse()?;
    let temp = temp / 1000.0; // Convert temperature from millidegrees Celsius to degrees Celsius
    Ok(temp)
}

fn main() {
    let zone = 0; // Specify the thermal zone (0 or 1) for the CPU temperature you want to read

    match cpu_temp(zone) {
        Ok(temp) => println!("CPU temperature (zone {}): {:.2}°C", zone, temp),
        Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
    }
}
