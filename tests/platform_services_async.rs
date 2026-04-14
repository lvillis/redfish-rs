#![cfg(feature = "_async")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use http::StatusCode;
use redfish::{Auth, Client, ResetType};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

#[tokio::test]
async fn account_service_crud_and_pagination_work() {
    let server = MockServer::start().await;

    // First page with nextLink.
    Mock::given(method("GET"))
        .and(path("/redfish/v1/AccountService/Accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
                "Members":[{"@odata.id":"/redfish/v1/AccountService/Accounts/1"}],
                "Members@odata.count":2,
                "Members@odata.nextLink":"/redfish/v1/AccountService/Accounts/next"
            }"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/redfish/v1/AccountService/Accounts/next"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
                "Members":[{"@odata.id":"/redfish/v1/AccountService/Accounts/2"}],
                "Members@odata.count":2
            }"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/redfish/v1/AccountService/Accounts"))
        .respond_with(
            ResponseTemplate::new(StatusCode::CREATED.as_u16())
                .insert_header("Location", "/redfish/v1/AccountService/Accounts/3")
                .set_body_raw(
                    r#"{"Id":"3","UserName":"u3","RoleId":"ReadOnly"}"#,
                    "application/json",
                ),
        )
        .mount(&server)
        .await;

    Mock::given(method("PATCH"))
        .and(path("/redfish/v1/AccountService/Accounts/3"))
        .respond_with(ResponseTemplate::new(StatusCode::OK.as_u16()).set_body_raw(
            r#"{"Id":"3","UserName":"u3","Enabled":false}"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/redfish/v1/AccountService/Accounts/3"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT.as_u16()))
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .unwrap()
        .auth(Auth::none())
        .build()
        .unwrap();

    let all = client.account_service().list_accounts_all().await.unwrap();
    assert_eq!(all.len(), 2);

    let created = client
        .account_service()
        .create_account(&redfish::AccountCreateRequest::new("u3", "pw").role_id("ReadOnly"))
        .await
        .unwrap();

    assert_eq!(created.status, StatusCode::CREATED);
    assert_eq!(
        created.location.as_deref(),
        Some("/redfish/v1/AccountService/Accounts/3")
    );
    assert_eq!(created.body.unwrap().id.as_deref(), Some("3"));

    let updated = client
        .account_service()
        .update_account("3", &redfish::AccountUpdateRequest::new().enabled(false))
        .await
        .unwrap();
    assert_eq!(updated.status, StatusCode::OK);

    client.account_service().delete_account("3").await.unwrap();
}

#[tokio::test]
async fn update_service_simple_update_returns_location() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/redfish/v1/UpdateService/Actions/UpdateService.SimpleUpdate",
        ))
        .respond_with(
            ResponseTemplate::new(StatusCode::ACCEPTED.as_u16())
                .insert_header("Location", "/redfish/v1/TaskService/Tasks/99")
                .set_body_raw(
                    r#"{"Task":"/redfish/v1/TaskService/Tasks/99"}"#,
                    "application/json",
                ),
        )
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri()).unwrap().build().unwrap();
    let resp = client
        .update_service()
        .simple_update(&redfish::SimpleUpdateRequest::new(
            "https://example.com/fw.bin",
        ))
        .await
        .unwrap();

    assert_eq!(resp.status, StatusCode::ACCEPTED);
    assert_eq!(
        resp.location.as_deref(),
        Some("/redfish/v1/TaskService/Tasks/99")
    );
}

struct TaskSeqResponder {
    calls: Arc<AtomicUsize>,
}

impl Respond for TaskSeqResponder {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let n = self.calls.fetch_add(1, Ordering::SeqCst);
        if n == 0 {
            ResponseTemplate::new(StatusCode::OK.as_u16()).set_body_raw(
                r#"{"Id":"99","TaskState":"Running","PercentComplete":10}"#,
                "application/json",
            )
        } else {
            ResponseTemplate::new(StatusCode::OK.as_u16()).set_body_raw(
                r#"{"Id":"99","TaskState":"Completed","PercentComplete":100}"#,
                "application/json",
            )
        }
    }
}

#[tokio::test]
async fn task_service_wait_for_task_polls_until_done() {
    let server = MockServer::start().await;

    let responder = TaskSeqResponder {
        calls: Arc::new(AtomicUsize::new(0)),
    };

    Mock::given(method("GET"))
        .and(path("/redfish/v1/TaskService/Tasks/99"))
        .respond_with(responder)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri()).unwrap().build().unwrap();

    let task = client
        .task_service()
        .wait_for_task(
            "/redfish/v1/TaskService/Tasks/99",
            Duration::from_millis(1),
            Duration::from_secs(1),
        )
        .await
        .unwrap();

    assert_eq!(task.task_state.as_deref(), Some("Completed"));
    assert!(task.is_done());
}

#[tokio::test]
async fn system_reset_action_works() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/redfish/v1/Systems/1/Actions/ComputerSystem.Reset"))
        .respond_with(ResponseTemplate::new(StatusCode::NO_CONTENT.as_u16()))
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri()).unwrap().build().unwrap();
    let resp = client
        .systems()
        .reset("1", ResetType::ForceRestart)
        .await
        .unwrap();
    assert_eq!(resp.status, StatusCode::NO_CONTENT);
}
