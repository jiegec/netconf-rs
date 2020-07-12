//! Get all interfaces
//! 
//! Reference:
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/interface.py
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/switchport.py

use crate::Connection;
use log::*;
use serde_derive::Deserialize;
use serde_xml_rs::from_str;
use std::io;

#[derive(Debug, Deserialize)]
struct RpcReply {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    pub top: Top,
}

#[derive(Debug, Deserialize)]
struct Top {
    #[serde(rename = "Ifmgr")]
    pub ifmgr: Ifmgr,
}

#[derive(Debug, Deserialize)]
pub struct Ifmgr {
    #[serde(rename = "Interfaces")]
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize)]
pub struct Interfaces {
    #[serde(rename = "Interface")]
    pub interface: Vec<Interface>,
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    #[serde(rename = "IfIndex")]
    pub index: usize,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "PVID")]
    pub port_vlan_id: Option<usize>,
    #[serde(rename = "ConfigMTU")]
    pub mtu: Option<usize>,
    /// 1 means access port
    /// 2 means trunk port
    #[serde(rename = "LinkType")]
    pub link_type: Option<usize>,
    /// 1 means bridged
    /// 2 means routed
    #[serde(rename = "PortLayer")]
    pub port_layer: Option<usize>,
}

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
    info!("{:#?}", reply.data.top.ifmgr.interfaces);
    Ok(reply.data.top.ifmgr.interfaces)
}
