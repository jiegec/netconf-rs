use log::*;
use memmem::{Searcher, TwoWaySearcher};
use ssh2::Session;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct SSHTransport {
    session: Session,
}

impl SSHTransport {
    pub fn connect(addr: &str, user_name: &str, password: &str) -> io::Result<SSHTransport> {
        let tcp = TcpStream::connect(addr)?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        sess.userauth_password(user_name, password)?;
        if sess.authenticated() {
            let mut res = SSHTransport { session: sess };
            res.hello()?;
            Ok(res)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    fn hello(&mut self) -> io::Result<()> {
        let mut channel = self.session.channel_session()?;
        channel.subsystem("netconf")?;
        info!("Get capabilities of NetConf server");
        write!(
            &mut channel,
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
        "#
        )?;
        let mut buffer = [0u8; 128];
        let mut s = Vec::new();
        let search = TwoWaySearcher::new("]]>]]>".as_bytes());
        while search.search_in(&s).is_none() {
            let bytes = channel.read(&mut buffer)?;
            s.extend(&buffer[..bytes]);
        }
        let resp = String::from_utf8(s).unwrap();
        info!("Got {}", resp);
        Ok(())
    }
}
