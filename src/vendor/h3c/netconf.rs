use super::{NetconfState, RpcReply};
use crate::Connection;
use log::*;
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
