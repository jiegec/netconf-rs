use crate::transport::Transport;
use log::*;
use markup5ever_rcdom::{NodeData, RcDom};
use std::io;
use xml5ever::driver::parse_document;
use xml5ever::tendril::stream::TendrilSink;
use xml5ever::tree_builder::TreeSink;

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
        let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(resp.as_str());
        let doc = &dom.document;
        // /hello/capabilities
        let hello = &doc.children.borrow()[1];
        let caps = &hello.children.borrow()[0];
        for node in caps.children.borrow().iter() {
            let text_node = &node.children.borrow()[0];
            match &text_node.data {
                &NodeData::Text { ref contents } => {
                    info!("capability {}", contents.borrow());
                }
                _ => {}
            }
        }
        //info!("Got {:?}", resp);
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
