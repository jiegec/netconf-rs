use super::{Mac, RpcReply};
use crate::Connection;
use log::*;
use serde_xml_rs::from_str;
use std::io;

/// Get YANG schema
pub fn get_mac_table(conn: &mut Connection) -> io::Result<Mac> {
    conn.transport.write_xml(&format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get>
        <filter type="subtree">
            <top xmlns="http://www.h3c.com/netconf/data:1.0">
                <MAC>
                    <MacUnicastTable>
                    </MacUnicastTable>
                </MAC>
            </top>
        </filter>
    </get>
</rpc>"#,
    ))?;
    let resp = conn.transport.read_xml()?;
    let reply: RpcReply = from_str(&resp.trim()).unwrap();
    debug!("{:?}", reply.data);
    Ok(reply.data.top.unwrap().mac.unwrap())
}
