#![cfg(feature = "async")]

use std::time::Duration;

use http::StatusCode;
use redfish::{Auth, Client, RetryPolicy};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn service_root_get_works() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/redfish/v1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"RedfishVersion":"1.9.0","UUID":"test"}"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .unwrap()
        .auth(Auth::none())
        .build()
        .unwrap();

    let root = client.service_root().get().await.unwrap();
    assert_eq!(root.redfish_version, "1.9.0");
    assert_eq!(root.uuid.as_deref(), Some("test"));
}

#[tokio::test]
async fn retries_on_503_for_idempotent_methods() {
    let server = MockServer::start().await;

    // First call -> 503, second call -> 200
    Mock::given(method("GET"))
        .and(path("/redfish/v1/Systems"))
        .respond_with(ResponseTemplate::new(503))
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

    let retry = RetryPolicy::default()
        .with_max_retries(2)
        .with_base_delay(Duration::from_millis(1))
        .with_max_delay(Duration::from_millis(1))
        .with_jitter(false);

    let client = Client::builder(&server.uri())
        .unwrap()
        .retry_policy(retry)
        .build()
        .unwrap();

    let systems = client.systems().list().await.unwrap();
    assert_eq!(systems.members.len(), 1);
}

#[tokio::test]
async fn session_create_parses_token_and_location() {
    let server = MockServer::start().await;

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

    let client = Client::builder(&server.uri()).unwrap().build().unwrap();
    let login = client.sessions().create("admin", "pw").await.unwrap();

    // We cannot (and should not) expose the raw secret here; but we can use it to build auth.
    let _auth = redfish::Auth::SessionToken(login.token.clone());
    assert_eq!(login.location, "/redfish/v1/SessionService/Sessions/1");
    assert_eq!(login.session.unwrap().id.as_deref(), Some("1"));
}
