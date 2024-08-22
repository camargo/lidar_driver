//! This file wraps [serialport::SerialPort] in a new struct called [Uart] so we can implement [embedded_io] HAL traits
//! for reading and writing to a serial port.

use embedded_io::{Error as EmbeddedIoError, ErrorKind, ErrorType, Read, Write};
use serialport;
use std::io;
use std::time::Duration;

/// UART struct that uses the [serialport] library for IO.
pub struct Uart {
    port: Box<dyn serialport::SerialPort>,
}

impl Uart {
    pub fn new(port_name: &str, baud_rate: u32, timeout: u64) -> Self {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_secs(timeout))
            .open()
            .expect(&format!("Failed to open {port_name}"));
        Self { port }
    }
}

impl ErrorType for Uart {
    type Error = IoError;
}

/// Wrap io::Error so we can implement [EmbeddedIoError] for it.
#[derive(Debug)]
pub struct IoError(io::Error);

/// Map [io] errors to [embedded_io] errors.
/// Most are just mapped to [ErrorKind::Other] since it's not a 1-1 mapping.
impl EmbeddedIoError for IoError {
    fn kind(&self) -> ErrorKind {
        match self.0.kind() {
            io::ErrorKind::TimedOut => ErrorKind::TimedOut,
            io::ErrorKind::WriteZero => ErrorKind::WriteZero,
            io::ErrorKind::Unsupported => ErrorKind::Unsupported,
            _ => ErrorKind::Other,
        }
    }
}

/// Implements the [embedded_io::Read] trait for [Uart] to allow for reading from a serial port.
impl Read for Uart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.port.read(buf).map_err(IoError)
    }
}

/// Implements the [embedded_io::Write] trait for [Uart] to allow for writing to a serial port.
impl Write for Uart {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.port.write(buf).map_err(IoError)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.port.flush().map_err(IoError)
    }
}
