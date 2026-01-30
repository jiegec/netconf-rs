//! Transports for NETCONF

use std::io;

#[cfg(feature = "ssh2")]
pub mod ssh;

#[cfg(feature = "russh")]
pub mod russh;

/// Trait for NETCONF transport
pub trait Transport: Send {
    fn read_xml(&mut self) -> io::Result<String>;
    fn write_xml(&mut self, data: &str) -> io::Result<()>;
}
