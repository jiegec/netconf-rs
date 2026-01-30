//! SSH transport using ssh2 library

use crate::transport::Transport;
use memmem::{Searcher, TwoWaySearcher};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use ssh2::{Channel, Session};

/// Configuration for SSH transport using ssh2 library
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
pub struct SSHTransport {
    #[allow(dead_code)]
    session: Session,
    channel: Channel,
    read_buffer: Vec<u8>,
}

impl SSHTransport {
    /// Connect with default configuration
    pub fn connect(addr: &str, user_name: &str, password: &str) -> io::Result<SSHTransport> {
        Self::connect_with_config(addr, user_name, password, &SSHConfig::default())
    }

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
