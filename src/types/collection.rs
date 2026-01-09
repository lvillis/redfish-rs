use serde::Deserialize;
use serde_json::{Map, Value};

use super::OdataId;

/// Generic Redfish collection.
///
/// Most Redfish collections look like:
/// - `Members`: list of objects (often `@odata.id` links)
/// - `Members@odata.count`: optional total
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct Collection<T = OdataId> {
    #[serde(rename = "@odata.id", default)]
    pub odata_id: Option<String>,

    #[serde(rename = "Name", default)]
    pub name: Option<String>,

    #[serde(rename = "Members", default)]
    pub members: Vec<T>,

    #[serde(rename = "Members@odata.count", default)]
    pub members_count: Option<u64>,

    /// Pagination hint for collections. Some implementations use `Members@odata.nextLink`.
    #[serde(rename = "Members@odata.nextLink", default)]
    pub members_next_link: Option<String>,

    /// Pagination hint for collections. Some implementations may use `@odata.nextLink`.
    #[serde(rename = "@odata.nextLink", default)]
    pub odata_next_link: Option<String>,

    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl<T> Collection<T> {
    /// Returns the next-link pagination URI, if present.
    pub fn next_link(&self) -> Option<&str> {
        self.members_next_link
            .as_deref()
            .or(self.odata_next_link.as_deref())
    }
}
