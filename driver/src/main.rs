use embedded_io::{Read, Write};
use emulator::{LidarHealthData, LidarMode, LidarScanData};

mod uart;
use uart::*;

// Lidar will never send more than 16 bytes of data.
const MAX_SENSOR_IN_BUFFER_SIZE: usize = 16;

// Commands are 4 bytes.
const MAX_COMMAND_IN_BUFFER_SIZE: usize = 4;

/// Main Lidar struct that contains a generic PORT type that is [embedded_io::Read] and [embedded_io::Write].
/// The PORT generic allows for more easily swapping out other communication protocols as long as they implement the [embedded_io] traits.
pub struct Lidar<PORT>
where
    PORT: Read + Write,
{
    // Port to receive commands.
    pub command_in_port: PORT,

    // Input buffer we read commands into.
    pub command_in_buffer: [u8; MAX_COMMAND_IN_BUFFER_SIZE],

    // Port to send commands.
    pub command_out_port: PORT,

    // Input buffer we read into directly from the Lidar.
    pub sensor_in_buffer: [u8; MAX_SENSOR_IN_BUFFER_SIZE],

    // Port to receive sensor data.
    pub sensor_in_port: PORT,
}

impl<PORT> Lidar<PORT>
where
    PORT: Read + Write,
{
    pub fn new(command_in_port: PORT, command_out_port: PORT, sensor_in_port: PORT) -> Self {
        Lidar {
            command_in_port,
            command_in_buffer: [0u8; MAX_COMMAND_IN_BUFFER_SIZE],
            command_out_port,
            sensor_in_buffer: [0u8; MAX_SENSOR_IN_BUFFER_SIZE],
            sensor_in_port,
        }
    }

    /// Receive input data from the Lidar.
    fn input(&mut self) {
        self.sensor_in_buffer = [0u8; MAX_SENSOR_IN_BUFFER_SIZE];

        match self.sensor_in_port.read(&mut self.sensor_in_buffer) {
            Ok(_) => {
                println!("[driver] Received input from the sensor");
            }
            Err(_) => {
                println!("[driver] No input received from the sensor");
            }
        }
    }

    /// Process input data by decoding it.
    fn process(&mut self) {
        let mode = bincode::deserialize::<LidarMode>(&self.sensor_in_buffer[0..4]);

        match mode {
            Ok(LidarMode::Standby) => {
                let lidar_health_data =
                    bincode::deserialize::<LidarHealthData>(&self.sensor_in_buffer).unwrap();
                println!("[driver] {:#?}", lidar_health_data);
            }
            Ok(LidarMode::Scan) => {
                let lidar_scan_data =
                    bincode::deserialize::<LidarScanData>(&self.sensor_in_buffer).unwrap();
                println!("[driver] {:#?}", lidar_scan_data);
            }
            Err(e) => {
                println!("[driver] Could not process input, mode not found: {e}")
            }
        }
    }

    /// Process commands and output them to the sensor.
    fn output(&mut self) {
        match self.command_in_port.read(&mut self.command_in_buffer) {
            Ok(n) => {
                println!("[driver] Received {n} command bytes");
                self.command_out_port
                    .write_all(&self.command_in_buffer)
                    .unwrap();
            }
            Err(_) => {
                println!("[driver] No commands received");
            }
        }
    }
}

fn main() {
    let command_in_port_name = "/tmp/command_in"; // Name of interface to receive commands from.
    let command_out_port_name = "/tmp/driver_out"; // Name of interface to send sensor commands to.
    let sensor_in_port_name = "/tmp/driver_in"; // Name of interface to receive sensor data from.
    let baud_rate = 9600; // 960 bytes/second.
    let timeout = 1; // How long between each port access.

    let mut lidar = Lidar::new(
        Uart::new(command_in_port_name, baud_rate, timeout),
        Uart::new(command_out_port_name, baud_rate, timeout),
        Uart::new(sensor_in_port_name, baud_rate, timeout),
    );

    // Single polling loop for the sensor.
    loop {
        lidar.input();
        lidar.process();
        lidar.output();
    }
}
