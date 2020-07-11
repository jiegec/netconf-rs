use crate::transport::Transport;
use log::*;
use std::io;

pub mod transport;

pub struct Connection {
    transport: Box<dyn Transport + 'static>,
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
        info!("Get capabilities of NetConf server");
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
        info!("Got {}", resp);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
