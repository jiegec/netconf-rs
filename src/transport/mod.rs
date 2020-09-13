//! Transports for NETCONF

use std::io;

pub mod ssh;

/// Trait for NETCONF transport
pub trait Transport: Send {
    fn read_xml(&mut self) -> io::Result<String>;
    fn write_xml(&mut self, data: &str) -> io::Result<()>;
}
