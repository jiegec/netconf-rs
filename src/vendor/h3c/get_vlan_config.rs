//! Get list of VLAN
//!
//! Reference:
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/vlan.py

use crate::xml::from_str;
use crate::Connection;
use log::*;
use serde_derive::Deserialize;
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
    #[serde(rename = "VLAN")]
    pub vlan: VlanConfig,
}

#[derive(Debug, Deserialize)]
pub struct VlanConfig {
    #[serde(rename = "VLANs")]
    pub vlans: Vlans,
}

#[derive(Debug, Deserialize)]
pub struct Vlans {
    #[serde(rename = "VLANID")]
    pub vlans: Vec<Vlan>,
}

#[derive(Debug, Deserialize)]
pub struct Vlan {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

/// Get all VLAN configs.
pub fn get_vlan_config(conn: &mut Connection) -> io::Result<VlanConfig> {
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
                <VLAN/>
            </top>
        </filter>
    </get-config>
</rpc>"#,
    )?;
    let resp = conn.transport.read_xml()?;
    let reply: RpcReply = from_str(resp.trim()).unwrap();
    debug!("{:#?}", reply.data.top.vlan.vlans);
    Ok(reply.data.top.vlan)
}
