use crate::transport::Transport;
use log::*;
use serde_derive::Deserialize;
use serde_xml_rs::from_str;
use std::io;

pub mod transport;
pub mod vendor;

#[derive(Debug, Deserialize)]
struct Hello {
    pub capabilities: Capabilities,
}

#[derive(Debug, Deserialize)]
struct Capabilities {
    pub capability: Vec<String>,
}

/// A connection to NETCONF server
pub struct Connection {
    pub(crate) transport: Box<dyn Transport + 'static>,
}

impl Connection {
    pub fn new(transport: impl Transport + 'static) -> io::Result<Connection> {
        let mut res = Connection {
            transport: Box::from(transport),
        };
        res.hello()?;
        Ok(res)
    }

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
        let hello: Hello = from_str(&resp).unwrap();
        debug!("{:#?}", hello);
        Ok(())
    }

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
