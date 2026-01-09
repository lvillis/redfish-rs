use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// BootSourceOverrideEnabled is modeled as a string newtype to preserve unknown values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BootSourceOverrideEnabled(String);

impl BootSourceOverrideEnabled {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn disabled() -> Self {
        Self::new("Disabled")
    }

    pub fn once() -> Self {
        Self::new("Once")
    }

    pub fn continuous() -> Self {
        Self::new("Continuous")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// BootSourceOverrideTarget is modeled as a string newtype to preserve unknown values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BootSourceOverrideTarget(String);

impl BootSourceOverrideTarget {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn none() -> Self {
        Self::new("None")
    }

    pub fn pxe() -> Self {
        Self::new("Pxe")
    }

    pub fn hdd() -> Self {
        Self::new("Hdd")
    }

    pub fn cd() -> Self {
        Self::new("Cd")
    }

    pub fn bios_setup() -> Self {
        Self::new("BiosSetup")
    }

    pub fn uefi_shell() -> Self {
        Self::new("UefiShell")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Boot configuration block used by `ComputerSystem` updates.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Boot {
    #[serde(
        rename = "BootSourceOverrideEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_source_override_enabled: Option<BootSourceOverrideEnabled>,

    #[serde(
        rename = "BootSourceOverrideTarget",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_source_override_target: Option<BootSourceOverrideTarget>,

    /// Many implementations use `UEFI` or `Legacy`.
    #[serde(
        rename = "BootSourceOverrideMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_source_override_mode: Option<String>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl Boot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

/// PATCH payload for `ComputerSystem`.
///
/// Typical path: `PATCH /redfish/v1/Systems/{id}`
#[derive(Debug, Clone, Serialize, Default)]
pub struct ComputerSystemUpdateRequest {
    #[serde(rename = "Boot", skip_serializing_if = "Option::is_none")]
    pub boot: Option<Boot>,

    #[serde(rename = "AssetTag", skip_serializing_if = "Option::is_none")]
    pub asset_tag: Option<String>,

    #[serde(rename = "IndicatorLED", skip_serializing_if = "Option::is_none")]
    pub indicator_led: Option<String>,

    /// Optional vendor extensions.
    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
}

impl ComputerSystemUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
