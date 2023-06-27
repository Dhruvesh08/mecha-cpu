use std::{fs, num::ParseFloatError, io};

pub fn cpu_temp(zone: u8) -> Result<f64, io::Error> {
    let zone_path = format!("/sys/class/thermal/thermal_zone{}/temp", zone);
    let temp_string = fs::read_to_string(zone_path)?;

    let temp: f64 = temp_string
        .trim()
        .parse()
        .map_err(|e: ParseFloatError| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let temp = temp / 1000.0; // Convert temperature from millidegrees Celsius to degrees Celsius
    Ok(temp)
}


