//! # netconf-rs
//!
//! A Rust library for NETCONF protocol ([RFC 6241](https://datatracker.ietf.org/doc/html/rfc6241)).
//!
//! NETCONF is a network management protocol defined by the IETF. It provides mechanisms to
//! install, manipulate, and delete the configuration of network devices. Its operations are
//! realized as Remote Procedure Calls (RPCs) encoded in XML.
//!
//! ## Features
//!
//! - **Multiple SSH backends**: Choose between `ssh2` (C-based) or `russh` (pure Rust) via Cargo features
//! - **Flexible XML parsing**: Supports both `serde-xml-rs` and `quick-xml` backends
//! - **Configurable security**: Fine-grained control over SSH algorithms and timeouts
//! - **Async support**: The `russh` backend provides async/await support with Tokio
//!
//! ## Quick Start
//!
//! ### Basic usage with ssh2 backend
//!
//! ```ignore
//! use netconf_rs::transport::ssh::SSHTransport;
//! use netconf_rs::Connection;
//!
//! # fn main() -> std::io::Result<()> {
//! let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
//! let mut conn = Connection::new(transport)?;
//! let config = conn.get_config()?;
//! println!("{}", config);
//! # Ok(())
//! # }
//! ```
//!
//! *Note: The ssh2 backend requires the `ssh2` feature to be enabled.*
//!
//! ### Using the russh backend
//!
//! ```ignore
//! use netconf_rs::transport::russh::RusshTransport;
//! use netconf_rs::Connection;
//!
//! # fn main() -> std::io::Result<()> {
//! let transport = RusshTransport::connect_password("192.168.1.1:830", "admin", "password")?;
//! let mut conn = Connection::new(transport)?;
//! let config = conn.get_config()?;
//! println!("{}", config);
//! # Ok(())
//! # }
//! ```
//!
//! *Note: The russh backend requires the `russh` feature to be enabled.*
//!
//! ## Cargo Features
//!
//! - **`ssh2`** (default): SSH transport using the ssh2 library
//! - **`russh`**: SSH transport using the russh library (pure Rust, async)
//! - **`serde-xml`** (default): XML parsing with serde-xml-rs
//! - **`quick-xml`**: XML parsing with quick-xml (faster alternative)
//!
//! ## Feature Combinations
//!
//! You can mix and match SSH and XML backends:
//!
//! ```toml
//! # Use ssh2 + serde-xml (default)
//! netconf-rs = "0.2"
//!
//! # Use russh + quick-xml
//! netconf-rs = { version = "0.2", features = ["russh", "quick-xml"] }
//!
//! # Use all features
//! netconf-rs = { version = "0.2", features = ["ssh2", "russh", "serde-xml", "quick-xml"] }
//! ```
//!
//! ## Transport Layer
//!
//! The library provides a `Transport` trait that abstracts the underlying communication layer.
//! Currently, two implementations are available:
//!
//! - **`SSHTransport`** (in `transport::ssh` module): Synchronous SSH client using ssh2
//! - **`RusshTransport`** (in `transport::russh` module): Asynchronous SSH client using russh
//!
//! Both support password and key-based authentication.

use crate::transport::Transport;
use crate::xml::from_str;
use log::*;
use serde_derive::Deserialize;
use std::io;

pub mod transport;
pub mod vendor;
pub mod xml;

#[derive(Debug, Deserialize)]
struct Hello {
    #[allow(dead_code)]
    pub capabilities: Capabilities,
}

#[derive(Debug, Deserialize)]
struct Capabilities {
    #[allow(dead_code)]
    pub capability: Vec<String>,
}

/// A connection to a NETCONF server
///
/// This struct represents an active NETCONF session with a remote device.
/// It handles the NETCONF protocol handshake (hello message exchange) and
/// provides methods for sending RPCs to the server.
///
/// The connection uses a transport layer (SSH or others) for communication,
/// abstracted through the `Transport` trait.
///
/// # Examples
///
/// ```ignore
/// use netconf_rs::transport::ssh::SSHTransport;
/// use netconf_rs::Connection;
///
/// # fn main() -> std::io::Result<()> {
/// let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
/// let mut conn = Connection::new(transport)?;
/// # Ok(())
/// # }
/// ```
///
/// *Note: This example requires the `ssh2` feature to be enabled.*
pub struct Connection {
    pub(crate) transport: Box<dyn Transport + Send + 'static>,
}

impl Connection {
    /// Creates a new NETCONF connection using the specified transport
    ///
    /// This method establishes a new NETCONF session by:
    /// 1. Wrapping the transport in a heap-allocated box
    /// 2. Performing the NETCONF hello handshake to exchange capabilities
    ///
    /// # Arguments
    ///
    /// * `transport` - Any type implementing the `Transport` trait (e.g., `SSHTransport`, `RusshTransport`)
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Connection` or an `io::Error` if the handshake fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use netconf_rs::transport::ssh::SSHTransport;
    /// use netconf_rs::Connection;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
    /// let mut conn = Connection::new(transport)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// *Note: This example requires the `ssh2` feature to be enabled.*
    pub fn new(transport: impl Transport + 'static) -> io::Result<Connection> {
        let mut res = Connection {
            transport: Box::from(transport),
        };
        res.hello()?;
        Ok(res)
    }

    /// Performs the NETCONF hello handshake
    ///
    /// This method sends a hello message to the server to exchange capabilities
    /// and establish the NETCONF session. The server responds with its own hello
    /// message listing supported capabilities.
    ///
    /// This is called automatically during `Connection::new()` and typically not
    /// called directly by users.
    fn hello(&mut self) -> io::Result<()> {
        debug!("Get capabilities of NetConf server");
        self.transport.write_xml(
            r#"
<?xml version="1.0" encoding="UTF-8"?>
<hello xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <capabilities>
        <capability>
            urn:ietf:params:netconf:base:1.0
        </capability>
    </capabilities>
</hello>
]]>]]>
        "#,
        )?;
        let resp = self.transport.read_xml()?;
        let hello: Hello = from_str(resp.trim()).unwrap();
        debug!("{:#?}", hello);
        Ok(())
    }

    /// Retrieves the running configuration from the NETCONF server
    ///
    /// This method sends a `<get-config>` RPC to retrieve the running configuration
    /// datastore from the server. The configuration is returned as an XML string.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configuration XML as a string, or an `io::Error`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use netconf_rs::Connection;
    /// # fn main() -> std::io::Result<()> {
    /// # let mut conn = Connection::new(netconf_rs::transport::ssh::SSHTransport::connect(
    /// #     "192.168.1.1:830", "admin", "password"
    /// # )?)?;
    /// let config = conn.get_config()?;
    /// println!("Running config:\n{}", config);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// *Note: This example requires the `ssh2` feature to be enabled.*
    pub fn get_config(&mut self) -> io::Result<String> {
        self.transport.write_xml(
            r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get-config>
        <source>
            <running/>
        </source>
    </get-config>
</rpc>
        "#,
        )?;
        let resp = self.transport.read_xml()?;
        Ok(resp)
    }
}
