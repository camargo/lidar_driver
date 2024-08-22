//! Common types exported as a library to be easily shared/used by driver.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Lidar is only ever in one of two modes: Standby or Scan.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum LidarMode {
    /// Standby mode Lidar is waiting to perform a scan, and sending health packets.
    Standby = 0x00,

    // Scan mode Lidar is sending scan data: x, y, z positions.
    Scan = 0x01,
}

/// Implement a nice display function for [LidarMode].
impl fmt::Display for LidarMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode_str = match self {
            LidarMode::Standby => "Standby",
            LidarMode::Scan => "Scan",
        };
        write!(f, "{}", mode_str)
    }
}

/// Struct representing Lidar health.
/// Currently the only health data is the sensors temperature.
#[derive(Debug, Deserialize, Serialize)]
pub struct LidarHealthData {
    /// Current mode the sensor is in.
    pub mode: LidarMode,

    /// Number of commands that have been sent to the Lidar.
    pub command_count: u32,

    // Current temperature in fahrenheit of the sensor.
    pub temp: f32,

    /// Current voltage the sensor is receiving.
    pub voltage: f32,
}

/// Struct representing Lidar scan data: x, y, z position of the Lidar laser point.
#[derive(Debug, Deserialize, Serialize)]
pub struct LidarScanData {
    /// Current mode the sensor is in.
    pub mode: LidarMode,

    /// Assume x, y, z are in an Earth-centered inertial (ECI) coordinate frame.
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
