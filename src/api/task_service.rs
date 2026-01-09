use std::time::{Duration, Instant};

use http::Method;

use crate::types::{Collection, OdataId, Task, TaskService};
use crate::{Error, RequestContext, Result};

#[cfg(feature = "blocking")]
use crate::BlockingClient;
#[cfg(feature = "async")]
use crate::Client;

/// Access `TaskService`.
#[derive(Debug, Clone, Copy)]
pub struct TaskServiceService<'a, C> {
    client: &'a C,
}

impl<'a, C> TaskServiceService<'a, C> {
    pub(crate) fn new(client: &'a C) -> Self {
        Self { client }
    }
}

#[cfg(feature = "async")]
impl<'a> TaskServiceService<'a, Client> {
    /// `GET /redfish/v1/TaskService`
    pub async fn get(&self) -> Result<TaskService> {
        let url = self.client.redfish_url(&["TaskService"])?;
        self.client.get_json(url).await
    }

    /// `GET /redfish/v1/TaskService/Tasks`
    pub async fn list_tasks(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["TaskService", "Tasks"])?;
        self.client.get_json(url).await
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all task members.
    pub async fn list_tasks_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/TaskService/Tasks")
            .await
    }

    /// `GET /redfish/v1/TaskService/Tasks/{id}`
    pub async fn get_task(&self, task_id: &str) -> Result<Task> {
        let url = self
            .client
            .redfish_url(&["TaskService", "Tasks", task_id])?;
        self.client.get_json(url).await
    }

    /// Fetch a task by URI (absolute or relative).
    pub async fn get_task_uri(&self, task_uri: &str) -> Result<Task> {
        self.client.get_uri(task_uri).await
    }

    /// Poll a task resource until it completes (or the timeout is hit).
    ///
    /// This is useful for actions that return `202 Accepted` with a task `Location`.
    pub async fn wait_for_task(
        &self,
        task_uri: &str,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Task> {
        let start = Instant::now();
        loop {
            let task = self.get_task_uri(task_uri).await?;
            if task.is_done() {
                return Ok(task);
            }

            if start.elapsed() >= timeout {
                let ctx = RequestContext::new(Method::GET, task_uri.to_string());
                return Err(Error::timeout(
                    ctx,
                    "Task did not complete within the specified timeout",
                    None,
                ));
            }

            if !poll_interval.is_zero() {
                tokio::time::sleep(poll_interval).await;
            }
        }
    }

    /// `DELETE /redfish/v1/TaskService/Tasks/{id}` (if supported by the implementation).
    pub async fn delete_task(&self, task_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["TaskService", "Tasks", task_id])?;
        self.client.delete_raw(url).await?;
        Ok(())
    }
}

#[cfg(feature = "blocking")]
impl<'a> TaskServiceService<'a, BlockingClient> {
    /// `GET /redfish/v1/TaskService`
    pub fn get(&self) -> Result<TaskService> {
        let url = self.client.redfish_url(&["TaskService"])?;
        self.client.get_json(url)
    }

    /// `GET /redfish/v1/TaskService/Tasks`
    pub fn list_tasks(&self) -> Result<Collection<OdataId>> {
        let url = self.client.redfish_url(&["TaskService", "Tasks"])?;
        self.client.get_json(url)
    }

    /// Convenience helper: follow `Members@odata.nextLink` and return all task members.
    pub fn list_tasks_all(&self) -> Result<Vec<OdataId>> {
        self.client
            .collect_all_collection_members("/redfish/v1/TaskService/Tasks")
    }

    /// `GET /redfish/v1/TaskService/Tasks/{id}`
    pub fn get_task(&self, task_id: &str) -> Result<Task> {
        let url = self
            .client
            .redfish_url(&["TaskService", "Tasks", task_id])?;
        self.client.get_json(url)
    }

    /// Fetch a task by URI (absolute or relative).
    pub fn get_task_uri(&self, task_uri: &str) -> Result<Task> {
        self.client.get_uri(task_uri)
    }

    /// Poll a task resource until it completes (or the timeout is hit).
    pub fn wait_for_task(
        &self,
        task_uri: &str,
        poll_interval: Duration,
        timeout: Duration,
    ) -> Result<Task> {
        let start = Instant::now();
        loop {
            let task = self.get_task_uri(task_uri)?;
            if task.is_done() {
                return Ok(task);
            }

            if start.elapsed() >= timeout {
                let ctx = RequestContext::new(Method::GET, task_uri.to_string());
                return Err(Error::timeout(
                    ctx,
                    "Task did not complete within the specified timeout",
                    None,
                ));
            }

            if !poll_interval.is_zero() {
                std::thread::sleep(poll_interval);
            }
        }
    }

    /// `DELETE /redfish/v1/TaskService/Tasks/{id}` (if supported by the implementation).
    pub fn delete_task(&self, task_id: &str) -> Result<()> {
        let url = self
            .client
            .redfish_url(&["TaskService", "Tasks", task_id])?;
        self.client.delete_raw(url)?;
        Ok(())
    }
}
