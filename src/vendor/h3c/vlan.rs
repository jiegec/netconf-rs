//! Configure VLAN
//!
//! Reference:
//! https://github.com/HPENetworking/pyhpecw7/blob/master/pyhpecw7/features/vlan.py

use crate::Connection;
use log::*;
use std::io;

/// Create VLAN
pub fn create_vlan(conn: &mut Connection, id: usize, desc: &str) -> io::Result<()> {
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
    debug!("Got {}", resp);
    Ok(())
}

/// Set port to VLAN access
pub fn set_vlan_access_port(conn: &mut Connection, id: usize, vlan: usize) -> io::Result<()> {
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
                    <AccessInterfaces>
                        <Interface>
                            <IfIndex>{}</IfIndex>
                            <PVID>{}</PVID>
                        </Interface>
                    </AccessInterfaces>
                </VLAN>
            </top>
         </config>
    </edit-config>
</rpc>"#,
        id, vlan
    ))?;
    let resp = conn.transport.read_xml()?;
    debug!("Got {}", resp);
    Ok(())
}

/// Set port to VLAN trunk
pub fn set_vlan_trunk_port(
    conn: &mut Connection,
    id: usize,
    permit_vlan_list: &[usize],
    pvid: Option<usize>,
) -> io::Result<()> {
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
                    <TrunkInterfaces>
                        <Interface>
                            <IfIndex>{}</IfIndex>
                            <PermitVlanList>{}</PermitVlanList>
                            <PVID>{}</PVID>
                        </Interface>
                    </TrunkInterfaces>
                </VLAN>
            </top>
         </config>
    </edit-config>
</rpc>"#,
        id,
        permit_vlan_list
            .iter()
            .map(|num| format!("{}", num))
            .collect::<Vec<String>>()
            .join(","),
        pvid.unwrap_or(1) // default pvid is VLAN 1
    ))?;
    let resp = conn.transport.read_xml()?;
    debug!("Got {}", resp);
    Ok(())
}
