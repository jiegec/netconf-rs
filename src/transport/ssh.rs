//! SSH transport

use crate::transport::Transport;
use memmem::{Searcher, TwoWaySearcher};
use ssh2::Channel;
use ssh2::Session;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

/// NETCONF over SSH
pub struct SSHTransport {
    session: Session,
    channel: Channel,
    read_buffer: Vec<u8>,
}

impl SSHTransport {
    pub fn connect(addr: &str, user_name: &str, password: &str) -> io::Result<SSHTransport> {
        let tcp = TcpStream::connect(addr)?;
        let mut sess = Session::new()?;
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
