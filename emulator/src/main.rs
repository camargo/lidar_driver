use emulator::*;
use rand::Rng;
use serialport;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let port_in_name = "/tmp/emulator_in";
    let port_out_name = "/tmp/emulator_out";
    let baud_rate = 9600;
    let timeout = 1;

    let mut current_mode = LidarMode::Standby;
    let mut rng = rand::thread_rng();
    let mut command_count = 0;
    let mut command_buffer = [0u8; 4];

    // Setup and open the input UART port.
    let mut port_in = serialport::new(port_in_name, baud_rate)
        .timeout(Duration::from_secs(timeout))
        .open()?;

    // Setup and open the output UART port.
    let mut port_out = serialport::new(port_out_name, baud_rate)
        .timeout(Duration::from_secs(timeout))
        .open()?;

    loop {
        // Read the port to receive a command.
        match port_in.read(&mut command_buffer) {
            Ok(n) => {
                if n > 0 {
                    let received_command = command_buffer[0];
                    command_count += 1;

                    match received_command {
                        0x00 => {
                            current_mode = LidarMode::Standby;
                            println!("[emulator] Lidar switched to Standby mode");
                        }
                        0x01 => {
                            current_mode = LidarMode::Scan;
                            println!("[emulator] Lidar switched to Scan mode");
                        }
                        _ => {
                            println!(
                                "[emulator] Received unknown command: 0x{:02X}",
                                received_command
                            );
                        }
                    }
                }
            }
            Err(_) => {
                println!("[emulator] No commands read from port");
            }
        }

        // Write the port with data depending on the current mode.
        match &current_mode {
            LidarMode::Standby => {
                // Setup some random health data.
                let lidar_health_data = LidarHealthData {
                    mode: current_mode,
                    command_count,
                    temp: rng.gen_range(60.0..61.0),
                    voltage: rng.gen_range(12.0..12.5),
                };
                let encoded: Vec<u8> = bincode::serialize(&lidar_health_data)?;

                port_out.write_all(&encoded)?;

                println!("[emulator] Sent health data");
            }
            LidarMode::Scan => {
                // Setup some random scan data.
                let lidar_scan_data = LidarScanData {
                    mode: current_mode,
                    x: rng.gen_range(123.0..126.0),
                    y: rng.gen_range(780.0..790.0),
                    z: rng.gen_range(345.0..348.0),
                };
                let encoded: Vec<u8> = bincode::serialize(&lidar_scan_data)?;

                port_out.write_all(&encoded)?;

                println!("[emulator] Sent scan data");
            }
        }
    }
}
