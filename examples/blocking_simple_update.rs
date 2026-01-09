//! Trigger a firmware update via `UpdateService.SimpleUpdate` (blocking client).

use std::time::Duration;

use redfish::{Auth, BlockingClient, SimpleUpdateRequest};

fn main() -> Result<(), redfish::Error> {
    let client = BlockingClient::builder("https://bmc.example.com")?
        .auth(Auth::basic("admin", "password"))
        .build()?;

    let resp = client
        .update_service()
        .simple_update(&SimpleUpdateRequest::new("https://example.com/fw.bin"))?;

    println!("Update response status: {}", resp.status);

    if let Some(task_uri) = resp.location.as_deref() {
        println!("Task: {}", task_uri);
        let task = client.task_service().wait_for_task(
            task_uri,
            Duration::from_secs(2),
            Duration::from_secs(300),
        )?;
        println!("Task done: {:?}", task.task_state);
    }

    Ok(())
}
