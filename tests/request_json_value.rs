#[cfg(feature = "async")]
mod async_tests {
    use http::Method;
    use redfish::Client;
    use serde_json::Value;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn request_json_value_returns_null_on_empty_body() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/redfish/v1/Actions/Oem.DoThing"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = Client::builder(&server.uri()).unwrap().build().unwrap();

        let v = client
            .request_json_value(Method::POST, "/redfish/v1/Actions/Oem.DoThing", None)
            .await
            .unwrap();

        assert_eq!(v, Value::Null);
    }
}

#[cfg(feature = "blocking")]
mod blocking_tests {
    use http::Method;
    use redfish::BlockingClient;
    use serde_json::Value;
    use tokio::runtime::Runtime;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn request_json_value_returns_null_on_empty_body() {
        let rt = Runtime::new().unwrap();
        let server = rt.block_on(MockServer::start());

        rt.block_on(async {
            Mock::given(method("POST"))
                .and(path("/redfish/v1/Actions/Oem.DoThing"))
                .respond_with(ResponseTemplate::new(204))
                .mount(&server)
                .await;
        });

        let client = BlockingClient::builder(&server.uri())
            .unwrap()
            .build()
            .unwrap();

        let v = client
            .request_json_value(Method::POST, "/redfish/v1/Actions/Oem.DoThing", None)
            .unwrap();

        assert_eq!(v, Value::Null);
    }
}
