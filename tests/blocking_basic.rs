#![cfg(feature = "_blocking")]

use std::time::Duration;

use http::StatusCode;
use redfish::{Auth, BlockingClient, RetryPolicy};
use tokio::runtime::Runtime;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn blocking_service_root_get_works() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/redfish/v1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"RedfishVersion":"1.9.0"}"#, "application/json"),
            )
            .mount(&server)
            .await;
    });

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .auth(Auth::none())
        .build()
        .unwrap();

    let root = client.service_root().get().unwrap();
    assert_eq!(root.redfish_version, "1.9.0");
}

#[test]
fn blocking_retries_on_503_for_idempotent_methods() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/redfish/v1/Systems"))
            .respond_with(ResponseTemplate::new(503))
            .up_to_n_times(1)
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/redfish/v1/Systems"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"Members":[{"@odata.id":"/redfish/v1/Systems/1"}],"Members@odata.count":1}"#,
                "application/json",
            ))
            .expect(1)
            .mount(&server)
            .await;
    });

    let retry = RetryPolicy::default()
        .with_max_retries(2)
        .with_base_delay(Duration::from_millis(1))
        .with_max_delay(Duration::from_millis(1))
        .with_jitter(false);

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .retry_policy(retry)
        .build()
        .unwrap();

    let systems = client.systems().list().unwrap();
    assert_eq!(systems.members.len(), 1);
}

#[test]
fn blocking_does_not_retry_on_500() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("GET"))
            .and(path("/redfish/v1/Systems"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;
    });

    let retry = RetryPolicy::default()
        .with_max_retries(2)
        .with_base_delay(Duration::from_millis(1))
        .with_max_delay(Duration::from_millis(1))
        .with_jitter(false);

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .retry_policy(retry)
        .build()
        .unwrap();

    let err = client.systems().list().expect_err("expected 500 error");
    assert_eq!(err.status(), Some(StatusCode::INTERNAL_SERVER_ERROR));

    let requests = rt
        .block_on(server.received_requests())
        .expect("request history");
    assert_eq!(requests.len(), 1);
}

#[test]
fn blocking_session_create_parses_token_and_location() {
    let rt = Runtime::new().unwrap();
    let server = rt.block_on(MockServer::start());

    rt.block_on(async {
        Mock::given(method("POST"))
            .and(path("/redfish/v1/SessionService/Sessions"))
            .respond_with(
                ResponseTemplate::new(StatusCode::CREATED.as_u16())
                    .insert_header("X-Auth-Token", "abc123")
                    .insert_header("Location", "/redfish/v1/SessionService/Sessions/1")
                    .set_body_raw(
                        r#"{"Id":"1","Name":"Session","UserName":"admin"}"#,
                        "application/json",
                    ),
            )
            .mount(&server)
            .await;
    });

    let client = BlockingClient::builder(&server.uri())
        .unwrap()
        .build()
        .unwrap();
    let login = client.sessions().create("admin", "pw").unwrap();

    let _auth = redfish::Auth::SessionToken(login.token.clone());
    assert_eq!(login.location, "/redfish/v1/SessionService/Sessions/1");
    assert_eq!(login.session.unwrap().id.as_deref(), Some("1"));
}
