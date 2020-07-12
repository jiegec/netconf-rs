use crate::Connection;
use log::*;
use serde_derive::Deserialize;
use serde_xml_rs::from_str;
use std::io;

#[derive(Debug, Deserialize)]
pub struct RpcReply {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub top: Top,
}

#[derive(Debug, Deserialize)]
pub struct Top {
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
    info!("Got {}", resp);
    let reply: RpcReply = from_str(&resp).unwrap();
    info!("{:#?}", reply.data.top.vlan.vlans);
    Ok(reply.data.top.vlan)
}
