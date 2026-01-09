#![cfg(feature = "blocking")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use http::StatusCode;
use redfish::{BlockingClient, ResetType};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[test]
fn blocking_account_service_crud_works() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("POST"))
            .and(path("/redfish/v1/AccountService/Accounts"))
            .respond_with(
                ResponseTemplate::new(StatusCode::CREATED.as_u16())
                    .insert_header("Location", "/redfish/v1/AccountService/Accounts/3")
                    .set_body_raw(r#"{"Id":"3","UserName":"u3"}"#, "application/json"),
            )
            .mount(&server)
            .await;

        Mock::given(method("PATCH"))
            .and(path("/redfish/v1/AccountService/Accounts/3"))
            .respond_with(
                ResponseTemplate::new(StatusCode::OK.as_u16())
                    .set_body_raw(r#"{"Id":"3","Enabled":false}"#, "application/json"),
            )
            .mount(&server)
            .await;

        Mock::given(method("DELETE"))
            .and(path("/redfish/v1/AccountService/Accounts/3"))
            .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT.as_u16()))
            .mount(&server)
            .await;
    });

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let created = client
        .account_service()
        .create_account(&redfish::AccountCreateRequest::new("u3", "pw"))
        .unwrap();

    assert_eq!(created.status, StatusCode::CREATED);
    assert_eq!(
        created.location.as_deref(),
        Some("/redfish/v1/AccountService/Accounts/3")
    );

    let updated = client
        .account_service()
        .update_account("3", &redfish::AccountUpdateRequest::new().enabled(false))
        .unwrap();

    assert_eq!(updated.status, StatusCode::OK);

    client.account_service().delete_account("3").unwrap();
}

struct TaskSeqResponder {
    calls: Arc<AtomicUsize>,
}

impl Respond for TaskSeqResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let n = self.calls.fetch_add(1, Ordering::SeqCst);
        if n == 0 {
            ResponseTemplate::new(StatusCode::OK.as_u16())
                .set_body_raw(r#"{"Id":"99","TaskState":"Running"}"#, "application/json")
        } else {
            ResponseTemplate::new(StatusCode::OK.as_u16())
                .set_body_raw(r#"{"Id":"99","TaskState":"Completed"}"#, "application/json")
        }
    }
}

#[test]
fn blocking_task_wait_for_task_polls_until_done() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    let responder = TaskSeqResponder {
        calls: Arc::new(AtomicUsize::new(0)),
    };

    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/redfish/v1/TaskService/Tasks/99"))
            .respond_with(responder)
            .mount(&server)
            .await;
    });

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .build()
        .unwrap();

    let task = client
        .task_service()
        .wait_for_task(
            "/redfish/v1/TaskService/Tasks/99",
            Duration::from_millis(1),
            Duration::from_secs(1),
        )
        .unwrap();

    assert_eq!(task.task_state.as_deref(), Some("Completed"));
    assert!(task.is_done());
}

#[test]
fn blocking_system_reset_action_works() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("POST"))
            .and(path("/redfish/v1/Systems/1/Actions/ComputerSystem.Reset"))
            .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT.as_u16()))
            .mount(&server)
            .await;
    });

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .build()
        .unwrap();
    let resp = client
        .systems()
        .reset("1", ResetType::ForceRestart)
        .unwrap();
    assert_eq!(resp.status, StatusCode::NO_CONTENT);
}
