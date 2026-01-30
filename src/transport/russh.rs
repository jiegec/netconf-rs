//! SSH transport using russh library

use crate::transport::Transport;
use memmem::{Searcher, TwoWaySearcher};
use russh::client;
use russh::keys::{load_secret_key, PrivateKeyWithHashAlg};
use russh::{Channel, ChannelMsg, Preferred};
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

/// NETCONF over SSH using russh library
pub struct RusshTransport {
    runtime: Runtime,
    channel: Channel<client::Msg>,
    read_buffer: Vec<u8>,
}

struct ClientHandler;

impl client::Handler for ClientHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true) // Accept all server keys for now
    }
}

impl RusshTransport {
    /// Connect using username and password
    pub fn connect_password(
        addr: &str,
        user_name: &str,
        password: &str,
    ) -> io::Result<RusshTransport> {
        let runtime = Runtime::new().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create runtime: {}", e),
            )
        })?;

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let handler = ClientHandler;

        let mut session = runtime
            .block_on(client::connect(config, addr, handler))
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Connection failed: {}", e))
            })?;

        let auth_result = runtime
            .block_on(session.authenticate_password(user_name, password))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Auth failed: {}", e)))?;

        if !auth_result.success() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Authentication failed",
            ));
        }

        let channel = runtime
            .block_on(session.channel_open_session())
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Channel open failed: {}", e))
            })?;

        runtime
            .block_on(channel.request_subsystem(true, "netconf"))
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Subsystem request failed: {}", e),
                )
            })?;

        Ok(RusshTransport {
            runtime,
            channel,
            read_buffer: Vec::new(),
        })
    }

    /// Connect using username and private key file
    pub fn connect_key(
        addr: &str,
        user_name: &str,
        key_file: &Path,
        passphrase: Option<&str>,
    ) -> io::Result<RusshTransport> {
        let runtime = Runtime::new().map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create runtime: {}", e),
            )
        })?;

        let key = load_secret_key(key_file, passphrase)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Key load failed: {}", e)))?;

        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            preferred: Preferred::default(),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let handler = ClientHandler;

        let mut session = runtime
            .block_on(client::connect(config, addr, handler))
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Connection failed: {}", e))
            })?;

        let key_with_alg = PrivateKeyWithHashAlg::new(Arc::new(key), None);
        let auth_result = runtime
            .block_on(session.authenticate_publickey(user_name, key_with_alg))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Auth failed: {}", e)))?;

        if !auth_result.success() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Authentication failed",
            ));
        }

        let channel = runtime
            .block_on(session.channel_open_session())
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Channel open failed: {}", e))
            })?;

        runtime
            .block_on(channel.request_subsystem(true, "netconf"))
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Subsystem request failed: {}", e),
                )
            })?;

        Ok(RusshTransport {
            runtime,
            channel,
            read_buffer: Vec::new(),
        })
    }
}

impl Transport for RusshTransport {
    fn read_xml(&mut self) -> io::Result<String> {
        let search = TwoWaySearcher::new("]]>]]>".as_bytes());
        while search.search_in(&self.read_buffer).is_none() {
            let msg = self
                .runtime
                .block_on(self.channel.wait())
                .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "Connection closed"))?;

            match msg {
                ChannelMsg::Data { ref data } => {
                    self.read_buffer.extend_from_slice(data);
                }
                _ => {}
            }
        }

        let pos = search.search_in(&self.read_buffer).unwrap();
        let resp = String::from_utf8(self.read_buffer[..pos].to_vec())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in response"))?;
        // 6: ]]>]]>
        self.read_buffer.drain(0..(pos + 6));
        Ok(resp)
    }

    fn write_xml(&mut self, data: &str) -> io::Result<()> {
        let message = format!("{}]]>]]>", data.trim());
        self.runtime
            .block_on(self.channel.data(message.as_bytes()))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Write error: {}", e)))?;
        Ok(())
    }
}
