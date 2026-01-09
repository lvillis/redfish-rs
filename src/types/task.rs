use serde::Deserialize;
use serde_json::{Map, Value};

use super::ResourceStatus;

/// Task resource.
///
/// Typically a member of `/redfish/v1/TaskService/Tasks`.
#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "@odata.type", default)]
    pub odata_type: Option<String>,

    #[serde(rename = "Id", default)]
    pub id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Description", default)]
    pub description: Option<String>,

    #[serde(rename = "TaskState", default)]
    pub task_state: Option<String>,

    #[serde(rename = "TaskStatus", default)]
    pub task_status: Option<String>,

    #[serde(rename = "PercentComplete", default)]
    pub percent_complete: Option<u32>,

    #[serde(rename = "StartTime", default)]
    pub start_time: Option<String>,

    #[serde(rename = "EndTime", default)]
    pub end_time: Option<String>,

    #[serde(rename = "Messages", default)]
    pub messages: Vec<TaskMessage>,

    #[serde(rename = "Status", default)]
    pub status: Option<ResourceStatus>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl Task {
    /// Returns `true` if the task appears to be finished.
    ///
    /// This uses common Redfish TaskState values.
    pub fn is_done(&self) -> bool {
        matches!(
            self.task_state.as_deref(),
            Some("Completed") | Some("Exception") | Some("Killed") | Some("Cancelled")
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskMessage {
    #[serde(rename = "MessageId", default)]
    pub message_id: Option<String>,

    #[serde(rename = "Message", default)]
    pub message: Option<String>,

    #[serde(rename = "RelatedProperties", default)]
    pub related_properties: Vec<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
