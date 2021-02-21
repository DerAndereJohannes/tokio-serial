//! Bindings for serial port I/O and futures
//!
//! This crate provides bindings between `mio_serial`, a mio crate for
//! serial port I/O, and `futures`.  The API is very similar to the
//! bindings in `mio_serial`
//!
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

// Re-export serialport types and traits from mio_serial
pub use mio_serial::{
    new, ClearBuffer, DataBits, Error, ErrorKind, FlowControl, MioSerialPort, Parity, SerialPort,
    SerialPortBuilder, StopBits,
};

/// A type for results generated by interacting with serial ports.
pub type Result<T> = mio_serial::Result<T>;

#[cfg(unix)]
/// Unix serial port
pub mod unix;
#[cfg(unix)]
pub use crate::unix::TTYPort;
