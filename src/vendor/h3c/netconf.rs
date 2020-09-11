use super::{NetconfState, RpcReply};
use crate::Connection;
use log::*;
use serde_derive::Deserialize;
use serde_xml_rs::from_str;
use std::io;

/// Get NETCONF information
pub fn get_netconf_information(conn: &mut Connection) -> io::Result<NetconfState> {
    conn.transport.write_xml(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get>
        <filter type="subtree">
            <netconf-state xmlns="urn:ietf:params:xml:ns:yang:ietf-netconf-monitoring">
            </netconf-state>
        </filter>
    </get>
</rpc>"#,
    )?;
    let resp = conn.transport.read_xml()?;
    let reply: RpcReply = from_str(&resp).unwrap();
    debug!("{:#?}", reply.data.netconf_state);
    Ok(reply.data.netconf_state.unwrap())
}

#[derive(Debug, Deserialize)]
struct GetSchemaRpcReply {
    data: String,
}

/// Get YANG schema
pub fn get_schema(
    conn: &mut Connection,
    id: &str,
    version: &str,
    format: &str,
) -> io::Result<String> {
    conn.transport.write_xml(&format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <get-schema xmlns='urn:ietf:params:xml:ns:yang:ietf-netconf-monitoring'>
        <identifier>{}</identifier>
        <version>{}</version>
        <format>{}</format>
  </get-schema>
</rpc>"#,
        id, version, format
    ))?;
    let resp = conn.transport.read_xml()?;
    let reply: GetSchemaRpcReply = from_str(&resp).unwrap();
    info!("{}", reply.data);
    Ok(reply.data)
}
