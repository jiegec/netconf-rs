use crate::Connection;
use log::*;
use std::io;

/// Create VLAN 
pub fn create_vlan(conn: &mut Connection, id: &str, desc: &str) -> io::Result<()> {
    conn.transport.write_xml(&format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc message-id="100"
    xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <edit-config>
        <target>
            <running/>
        </target>
        <config>
            <top xmlns="http://www.h3c.com/netconf/config:1.0">
                <VLAN>
                    <VLANs>
                        <VLANID>
                            <ID>{}</ID>
                            <Description>{}</Description>
                        </VLANID>
                    </VLANs>
                </VLAN>
            </top>
         </config>
    </edit-config>
</rpc>"#,
        id, desc
    ))?;
    let resp = conn.transport.read_xml()?;
    info!("Got {}", resp);
    Ok(())
}
