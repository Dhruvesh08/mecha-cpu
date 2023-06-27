use std::{fs, num::ParseFloatError, io, process::Command};

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

pub fn stress_test_cpu(cpu_max_prime: u64) {
    let output = Command::new("sysbench")
        .arg("--test=cpu")
        .arg("--num-threads=4")
        .arg(format!("--cpu-max-prime={}", cpu_max_prime))
        .arg("run")
        .output()
        .expect("Failed to execute sysbench command.");

    if output.status.success() {
        println!("Command executed successfully!");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed: {}", stderr);
    }
}


