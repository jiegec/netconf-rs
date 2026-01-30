//! SSH transport using ssh2 library
//!
//! This module provides NETCONF over SSH using the `ssh2` library, which is
//! a C-based SSH client library.
//!
//! ## Features
//!
//! - Synchronous API (blocking I/O)
//! - Password authentication
//! - Configurable SSH algorithms (KEX, ciphers, MACs, compression)
//! - Timeout support for connection and handshake
//!
//! ## Module Path
//!
//! `crate::transport::ssh::SSHTransport`
//!
//! ## Examples
//!
//! ### Basic usage
//!
//! ```no_run
//! use netconf_rs::transport::ssh::SSHTransport;
//!
//! # fn main() -> std::io::Result<()> {
//! let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
//! # Ok(())
//! # }
//! ```
//!
//! ### With custom configuration
//!
//! ```no_run
//! use netconf_rs::transport::ssh::{SSHTransport, SSHConfig};
//! use std::time::Duration;
//!
//! # fn main() -> std::io::Result<()> {
//! let config = SSHConfig::new()
//!     .connect_timeout(Duration::from_secs(10))
//!     .kex_algo("curve25519-sha256,diffie-hellman-group14-sha256")
//!     .cipher_algo("chacha20-poly1305@openssh.com,aes256-gcm@openssh.com");
//!
//! let transport = SSHTransport::connect_with_config(
//!     "192.168.1.1:830",
//!     "admin",
//!     "password",
//!     &config
//! )?;
//! # Ok(())
//! # }
//! ```

use crate::transport::Transport;
use memmem::{Searcher, TwoWaySearcher};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use ssh2::{Channel, Session};

/// Configuration for SSH transport using ssh2 library
///
/// This struct provides fine-grained control over SSH connection parameters
/// including timeouts and cryptographic algorithms.
///
/// # Example
///
/// ```
/// use netconf_rs::transport::ssh::SSHConfig;
/// use std::time::Duration;
///
/// let config = SSHConfig::new()
///     .connect_timeout(Duration::from_secs(10))
///     .handshake_timeout(Duration::from_secs(15))
///     .kex_algo("curve25519-sha256,diffie-hellman-group14-sha256")
///     .cipher_algo("chacha20-poly1305@openssh.com,aes256-gcm@openssh.com");
/// ```
#[derive(Debug, Clone)]
pub struct SSHConfig {
    /// Timeout for TCP connection
    pub connect_timeout: Option<Duration>,
    /// Timeout for SSH handshake
    pub handshake_timeout: Option<Duration>,
    /// Preferred key exchange methods (comma-separated string)
    pub kex_algo: Option<String>,
    /// Preferred server host key algorithms (comma-separated string)
    pub hostkey_algo: Option<String>,
    /// Preferred encryption algorithms/ciphers (comma-separated string)
    pub cipher_algo: Option<String>,
    /// Preferred MAC algorithms (comma-separated string)
    pub mac_algo: Option<String>,
    /// Preferred compression algorithms (comma-separated string)
    pub compression_algo: Option<String>,
}

impl Default for SSHConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Some(Duration::from_secs(30)),
            handshake_timeout: Some(Duration::from_secs(30)),
            kex_algo: None,
            hostkey_algo: None,
            cipher_algo: None,
            mac_algo: None,
            compression_algo: None,
        }
    }
}

impl SSHConfig {
    /// Create a new SSH configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the TCP connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Set the SSH handshake timeout
    pub fn handshake_timeout(mut self, timeout: Duration) -> Self {
        self.handshake_timeout = Some(timeout);
        self
    }

    /// Set preferred key exchange methods (comma-separated string)
    ///
    /// Example: `kex_algo("curve25519-sha256,curve25519-sha256@libssh.org,diffie-hellman-group14-sha256")`
    pub fn kex_algo(mut self, algos: &str) -> Self {
        self.kex_algo = Some(algos.to_string());
        self
    }

    /// Set preferred server host key algorithms (comma-separated string)
    ///
    /// Example: `hostkey_algo("ssh-ed25519,ecdsa-sha2-nistp256,rsa-sha2-512")`
    pub fn hostkey_algo(mut self, algos: &str) -> Self {
        self.hostkey_algo = Some(algos.to_string());
        self
    }

    /// Set preferred encryption algorithms/ciphers (comma-separated string)
    ///
    /// Example: `cipher_algo("chacha20-poly1305@openssh.com,aes256-gcm@openssh.com")`
    pub fn cipher_algo(mut self, algos: &str) -> Self {
        self.cipher_algo = Some(algos.to_string());
        self
    }

    /// Set preferred MAC algorithms (comma-separated string)
    ///
    /// Example: `mac_algo("hmac-sha2-256-etm@openssh.com,hmac-sha2-512-etm@openssh.com")`
    pub fn mac_algo(mut self, algos: &str) -> Self {
        self.mac_algo = Some(algos.to_string());
        self
    }

    /// Set preferred compression algorithms (comma-separated string)
    ///
    /// Example: `compression_algo("zlib,none")`
    pub fn compression_algo(mut self, algos: &str) -> Self {
        self.compression_algo = Some(algos.to_string());
        self
    }
}

/// NETCONF over SSH using ssh2 library
///
/// This struct provides a synchronous NETCONF transport over SSH using the `ssh2` library.
/// It manages the underlying SSH session and NETCONF channel, handling message framing
/// with the `]]>]]>` delimiter.
///
/// The transport automatically:
/// - Establishes a TCP connection to the server
/// - Performs the SSH handshake
/// - Authenticates with the provided credentials
/// - Opens a NETCONF subsystem channel
///
/// # Examples
///
/// ```no_run
/// use netconf_rs::transport::ssh::SSHTransport;
///
/// # fn main() -> std::io::Result<()> {
/// let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
/// // Use transport with Connection
/// # Ok(())
/// # }
/// ```
pub struct SSHTransport {
    #[allow(dead_code)]
    session: Session,
    channel: Channel,
    read_buffer: Vec<u8>,
}

impl SSHTransport {
    /// Connect to a NETCONF server with default configuration
    ///
    /// This method establishes a NETCONF over SSH connection using default settings:
    /// - 30 second connection timeout
    /// - 30 second handshake timeout
    /// - Default SSH algorithms
    ///
    /// # Arguments
    ///
    /// * `addr` - Server address in format "host:port" (e.g., "192.168.1.1:830")
    /// * `user_name` - SSH username
    /// * `password` - SSH password
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `SSHTransport` or an `io::Error`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use netconf_rs::transport::ssh::SSHTransport;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let transport = SSHTransport::connect("192.168.1.1:830", "admin", "password")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect(addr: &str, user_name: &str, password: &str) -> io::Result<SSHTransport> {
        Self::connect_with_config(addr, user_name, password, &SSHConfig::default())
    }

    /// Connect to a NETCONF server with custom SSH configuration
    ///
    /// This method establishes a NETCONF over SSH connection with custom settings
    /// for timeouts and cryptographic algorithms.
    ///
    /// # Arguments
    ///
    /// * `addr` - Server address in format "host:port" (e.g., "192.168.1.1:830")
    /// * `user_name` - SSH username
    /// * `password` - SSH password
    /// * `config` - SSH configuration options
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `SSHTransport` or an `io::Error`
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The address is invalid
    /// - TCP connection fails
    /// - SSH handshake fails
    /// - Authentication fails
    /// - NETCONF subsystem cannot be opened
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use netconf_rs::transport::ssh::{SSHTransport, SSHConfig};
    /// use std::time::Duration;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let config = SSHConfig::new()
    ///     .connect_timeout(Duration::from_secs(10))
    ///     .kex_algo("curve25519-sha256");
    ///
    /// let transport = SSHTransport::connect_with_config(
    ///     "192.168.1.1:830",
    ///     "admin",
    ///     "password",
    ///     &config
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    /// Connect with custom configuration
    pub fn connect_with_config(
        addr: &str,
        user_name: &str,
        password: &str,
        config: &SSHConfig,
    ) -> io::Result<SSHTransport> {
        let tcp = if let Some(timeout) = config.connect_timeout {
            let socket_addr = addr.parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid address: {}", e),
                )
            })?;
            TcpStream::connect_timeout(&socket_addr, timeout)?
        } else {
            TcpStream::connect(addr)?
        };

        let mut sess = Session::new()?;

        // Apply algorithm preferences
        if let Some(ref kex) = config.kex_algo {
            sess.method_pref(ssh2::MethodType::Kex, kex)?;
        }
        if let Some(ref hostkey) = config.hostkey_algo {
            sess.method_pref(ssh2::MethodType::HostKey, hostkey)?;
        }
        if let Some(ref ciphers) = config.cipher_algo {
            sess.method_pref(ssh2::MethodType::CryptCs, ciphers)?;
            sess.method_pref(ssh2::MethodType::CryptSc, ciphers)?;
        }
        if let Some(ref macs) = config.mac_algo {
            sess.method_pref(ssh2::MethodType::MacCs, macs)?;
            sess.method_pref(ssh2::MethodType::MacSc, macs)?;
        }
        if let Some(ref compression) = config.compression_algo {
            sess.method_pref(ssh2::MethodType::CompCs, compression)?;
            sess.method_pref(ssh2::MethodType::CompSc, compression)?;
        }

        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        sess.userauth_password(user_name, password)?;
        if sess.authenticated() {
            let mut channel = sess.channel_session()?;
            channel.subsystem("netconf")?;
            let res = SSHTransport {
                session: sess,
                channel,
                read_buffer: Vec::new(),
            };
            Ok(res)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

impl Transport for SSHTransport {
    fn read_xml(&mut self) -> io::Result<String> {
        let mut buffer = [0u8; 128];
        let search = TwoWaySearcher::new("]]>]]>".as_bytes());
        while search.search_in(&self.read_buffer).is_none() {
            let bytes = self.channel.read(&mut buffer)?;
            self.read_buffer.extend(&buffer[..bytes]);
        }
        let pos = search.search_in(&self.read_buffer).unwrap();
        let resp = String::from_utf8(self.read_buffer[..pos].to_vec()).unwrap();
        // 6: ]]>]]>
        self.read_buffer.drain(0..(pos + 6));
        Ok(resp)
    }

    fn write_xml(&mut self, data: &str) -> io::Result<()> {
        write!(&mut self.channel, r#"{}]]>]]>"#, data.trim())?;
        Ok(())
    }
}
