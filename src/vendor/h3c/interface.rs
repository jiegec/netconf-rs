//! Get all interfaces
//!
//! Reference:
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/interface.py
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/switchport.py

use super::{Interfaces, RpcReply};
use crate::Connection;
use log::*;
use serde_xml_rs::from_str;
use std::io;

/// Get all interfaces.
pub fn get_interfaces(conn: &mut Connection) -> io::Result<Interfaces> {
    conn.transport.write_xml(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get-config>
        <source>
            <running/>
        </source>
        <filter type="subtree">
            <top xmlns="http://www.h3c.com/netconf/config:1.0">
                <Ifmgr/>
            </top>
        </filter>
    </get-config>
</rpc>"#,
    )?;
    let resp = conn.transport.read_xml()?;
    let reply: RpcReply = from_str(&resp).unwrap();
    let top = reply.data.top.unwrap();
    debug!("{:#?}", top.ifmgr.interfaces);
    Ok(top.ifmgr.interfaces)
}
