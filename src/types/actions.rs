use serde::Serialize;

/// Reset action type used by several Redfish resources.
///
/// Values are defined by the Redfish schema and are typically PascalCase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetType {
    On,
    ForceOff,
    GracefulShutdown,
    GracefulRestart,
    ForceRestart,
    ForceOn,
    PushPowerButton,
    PowerCycle,
    /// Non-maskable interrupt.
    #[serde(rename = "Nmi")]
    Nmi,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ResetRequest {
    #[serde(rename = "ResetType")]
    pub reset_type: ResetType,
}
