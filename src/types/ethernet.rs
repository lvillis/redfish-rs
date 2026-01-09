use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::{Resource, ResourceStatus};

/// EthernetInterface resource.
///
/// Typical paths:
/// - `GET /redfish/v1/Systems/{id}/EthernetInterfaces/{eth_id}`
/// - `GET /redfish/v1/Managers/{id}/EthernetInterfaces/{eth_id}`
#[derive(Debug, Clone, Deserialize)]
pub struct EthernetInterface {
    #[serde(flatten)]
    pub resource: Resource,

    #[serde(rename = "InterfaceEnabled", default)]
    pub interface_enabled: Option<bool>,

    #[serde(rename = "MACAddress", default)]
    pub mac_address: Option<String>,

    #[serde(rename = "SpeedMbps", default)]
    pub speed_mbps: Option<u64>,

    #[serde(rename = "MTUSize", default)]
    pub mtu_size: Option<u64>,

    #[serde(rename = "HostName", default)]
    pub host_name: Option<String>,

    #[serde(rename = "FQDN", default)]
    pub fqdn: Option<String>,

    #[serde(rename = "IPv4Addresses", default)]
    pub ipv4_addresses: Vec<Ipv4Address>,

    #[serde(rename = "IPv6Addresses", default)]
    pub ipv6_addresses: Vec<Ipv6Address>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ipv4Address {
    #[serde(rename = "Address", default)]
    pub address: Option<String>,

    #[serde(rename = "SubnetMask", default)]
    pub subnet_mask: Option<String>,

    #[serde(rename = "Gateway", default)]
    pub gateway: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ipv6Address {
    #[serde(rename = "Address", default)]
    pub address: Option<String>,

    #[serde(rename = "PrefixLength", default)]
    pub prefix_length: Option<u8>,

    #[serde(rename = "AddressState", default)]
    pub address_state: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Patch payload for EthernetInterface.
///
/// Note: Redfish implementations vary widely here; this type models the most common fields
/// and allows OEM keys via `extra`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct EthernetInterfaceUpdateRequest {
    #[serde(rename = "InterfaceEnabled", skip_serializing_if = "Option::is_none")]
    pub interface_enabled: Option<bool>,

    #[serde(rename = "MTUSize", skip_serializing_if = "Option::is_none")]
    pub mtu_size: Option<u64>,

    #[serde(rename = "HostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,

    #[serde(rename = "FQDN", skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl EthernetInterfaceUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
