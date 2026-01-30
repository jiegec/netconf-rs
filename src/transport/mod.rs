//! Transports for NETCONF
//!
//! This module provides transport layer implementations for NETCONF protocol.
//! The `Transport` trait abstracts the underlying communication mechanism, allowing
//! different protocols (primarily SSH) to be used interchangeably.
//!
//! ## Available Transports
//!
//! - **SSH Transport** (`ssh` module): Synchronous SSH client using the `ssh2` library
//!   - Requires the `ssh2` feature
//!   - Module path: `crate::transport::ssh::SSHTransport`
//!
//! - **Russh Transport** (`russh` module): Asynchronous SSH client using the `russh` library
//!   - Requires the `russh` feature
//!   - Module path: `crate::transport::russh::RusshTransport`
//!   - Provides async/await support with Tokio runtime
//!
//! ## Transport Trait
//!
//! All transports implement the `Transport` trait which provides:
//! - `read_xml()`: Read a complete XML message (delimited by `]]>]]>`)
//! - `write_xml()`: Write an XML message with proper delimiters

use std::io;

#[cfg(feature = "ssh2")]
pub mod ssh;

#[cfg(feature = "russh")]
pub mod russh;

/// Trait for NETCONF transport layer
///
/// This trait defines the interface that all NETCONF transports must implement.
/// It provides methods for reading and writing XML messages with the proper
/// NETCONF message delimiters (`]]>]]>`).
///
/// The trait is designed to be transport-agnostic, allowing different protocols
/// (SSH, TCP, etc.) to be used for NETCONF communication.
///
/// # Required Methods
///
/// - `read_xml()`: Read a complete XML message from the transport
/// - `write_xml()`: Write an XML message to the transport
///
/// # Examples
///
/// Implementing a custom transport:
///
/// ```ignore
/// use std::io;
/// use netconf_rs::transport::Transport;
///
/// struct MyTransport;
///
/// impl Transport for MyTransport {
///     fn read_xml(&mut self) -> io::Result<String> {
///         // Read until ]]>]]> delimiter
///         Ok(String::from("<data/>"))
///     }
///
///     fn write_xml(&mut self, data: &str) -> io::Result<()> {
///         // Write data with ]]>]]> delimiter
///         Ok(())
///     }
/// }
/// ```
pub trait Transport: Send {
    /// Read a complete XML message from the transport
    ///
    /// This method should read data until the NETCONF message delimiter
    /// (`]]>]]>`) is encountered, then return the message content (without the delimiter).
    ///
    /// # Returns
    ///
    /// A `Result` containing the XML message as a string, or an `io::Error`
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The underlying connection is closed
    /// - Invalid UTF-8 data is received
    /// - An I/O error occurs
    fn read_xml(&mut self) -> io::Result<String>;

    /// Write an XML message to the transport
    ///
    /// This method should write the provided XML data to the transport,
    /// appending the NETCONF message delimiter (`]]>]]>`).
    ///
    /// # Arguments
    ///
    /// * `data` - The XML message to write (without delimiter)
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `io::Error`
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The underlying connection is closed
    /// - An I/O error occurs during writing
    fn write_xml(&mut self, data: &str) -> io::Result<()>;
}
