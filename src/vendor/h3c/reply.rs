use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RpcReply {
    pub data: Data,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Data {
    pub top: Option<Top>,
    #[serde(rename = "netconf-state")]
    pub netconf_state: Option<NetconfState>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Top {
    #[serde(rename = "Ifmgr")]
    pub ifmgr: Option<Ifmgr>,
    #[serde(rename = "MAC")]
    pub mac: Option<Mac>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Ifmgr {
    #[serde(rename = "Interfaces")]
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Interfaces {
    #[serde(rename = "Interface")]
    pub interface: Vec<Interface>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NetconfState {
    pub capabilities: Capabilities,
    pub schemas: Schemas,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Capabilities {
    pub capability: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Schemas {
    pub schema: Vec<Schema>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Schema {
    pub identifier: String,
    pub version: String,
    pub format: String,
    pub namespace: String,
    pub location: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Mac {
    #[serde(rename = "MacUnicastTable")]
    pub table: MacUnicastTable,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MacUnicastTable {
    #[serde(rename = "Unicast")]
    pub unicast: Vec<Unicast>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Unicast {
    #[serde(rename = "VLANID")]
    pub vlan_id: usize,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "PortIndex")]
    pub port_index: usize,
    #[serde(rename = "Status")]
    pub status: usize,
    #[serde(rename = "Aging")]
    pub aging: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_xml_rs::from_str;

    #[test]
    fn parse_mac_table() {
        let resp = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rpc-reply
	xmlns="urn:ietf:params:xml:ns:netconf:base:1.0" message-id="100">
	<data>
		<top
			xmlns="http://www.h3c.com/netconf/data:1.0">
			<MAC>
				<MacUnicastTable>
					<Unicast>
						<VLANID>1</VLANID>
						<MacAddress>12-34-56-78-90-AB</MacAddress>
						<PortIndex>634</PortIndex>
						<Status>2</Status>
						<Aging>true</Aging>
					</Unicast>
					<Unicast>
						<VLANID>2</VLANID>
						<MacAddress>11-11-11-11-11-11</MacAddress>
						<PortIndex>10</PortIndex>
						<Status>2</Status>
						<Aging>true</Aging>
					</Unicast>
				</MacUnicastTable>
			</MAC>
		</top>
	</data>
</rpc-reply> 
        "#;

        let reply: RpcReply = from_str(&resp).unwrap();
        assert_eq!(
            reply,
            RpcReply {
                data: Data {
                    top: Some(Top {
                        ifmgr: None,
                        mac: Some(Mac {
                            table: MacUnicastTable {
                                unicast: vec![
                                    Unicast {
                                        vlan_id: 1,
                                        mac_address: String::from("12-34-56-78-90-AB"),
                                        port_index: 634,
                                        status: 2,
                                        aging: true
                                    },
                                    Unicast {
                                        vlan_id: 2,
                                        mac_address: String::from("11-11-11-11-11-11"),
                                        port_index: 10,
                                        status: 2,
                                        aging: true
                                    }
                                ]
                            }
                        })
                    }),
                    netconf_state: None
                }
            }
        );
    }
}
