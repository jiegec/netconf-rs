use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcReply {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub top: Option<Top>,
    #[serde(rename = "netconf-state")]
    pub netconf_state: Option<NetconfState>,
}

#[derive(Debug, Deserialize)]
pub struct Top {
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

#[derive(Debug, Deserialize)]
pub struct NetconfState {
    pub capabilities: Capabilities,
    pub schemas: Schemas,
}

#[derive(Debug, Deserialize)]
pub struct Capabilities {
    pub capability: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Schemas {
    pub schema: Vec<Schema>,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub identifier: String,
    pub version: String,
    pub format: String,
    pub namespace: String,
    pub location: String,
}
