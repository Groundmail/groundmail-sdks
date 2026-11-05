use groundmail::{Client, Error, ListOptions, SendEmail};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client(server: &MockServer) -> Client {
    Client::with_base_url("gm_test", server.uri()).unwrap()
}

#[test]
fn refuses_a_key_that_isnt_ours() {
    assert!(matches!(
        Client::new("sk_nope"),
        Err(Error::InvalidApiKey(_))
    ));
}

#[tokio::test]
async fn sends_an_email() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails"))
        .and(header("authorization", "Bearer gm_test"))
        .and(header("idempotency-key", "order-1"))
        .and(body_json(json!({
            "from": "a@example.com",
            "to": ["b@example.com", "c@example.com"],
            "subject": "Hi",
            "html": "<p>Hi</p>",
            "tags": ["receipt"],
        })))
        .respond_with(
            ResponseTemplate::new(202)
                .set_body_json(json!({ "id": "msg_1", "status": "queued", "test": false })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let email = SendEmail::new("a@example.com", "b@example.com", "Hi")
        .to("c@example.com")
        .html("<p>Hi</p>")
        .field("tags", json!(["receipt"]))
        .idempotency_key("order-1");
    let sent = client(&server).await.emails().send(&email).await.unwrap();

    assert_eq!(sent.id, "msg_1");
    assert_eq!(sent.status, "queued");
    assert_eq!(sent.extra["test"], json!(false));
}

#[tokio::test]
async fn returns_api_errors_with_their_details() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/emails"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": { "message": "Domain not verified", "code": "forbidden", "upgrade_url": "https://example.com/up" }
        })))
        .mount(&server)
        .await;

    let email = SendEmail::new("a@example.com", "b@example.com", "Hi").text("Hi");
    let error = client(&server)
        .await
        .emails()
        .send(&email)
        .await
        .unwrap_err();

    match error {
        Error::Api {
            status,
            message,
            code,
            upgrade_url,
            ..
        } => {
            assert_eq!(status, 403);
            assert_eq!(message, "Domain not verified");
            assert_eq!(code.as_deref(), Some("forbidden"));
            assert_eq!(upgrade_url.as_deref(), Some("https://example.com/up"));
        }
        other => panic!("expected an API error, got {other:?}"),
    }
}

#[tokio::test]
async fn keeps_a_useful_message_when_the_error_isnt_json() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(502).set_body_string("Bad gateway"))
        .mount(&server)
        .await;

    let error = client(&server)
        .await
        .emails()
        .get("msg_1")
        .await
        .unwrap_err();
    assert_eq!(error.to_string(), "Request failed with status 502");
}

#[tokio::test]
async fn pages_through_lists() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/domains"))
        .and(query_param("limit", "10"))
        .and(query_param("cursor", "abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": [] })))
        .expect(1)
        .mount(&server)
        .await;

    let options = ListOptions {
        limit: Some(10),
        cursor: Some("abc".to_owned()),
    };
    client(&server)
        .await
        .domains()
        .list(&options)
        .await
        .unwrap();
}

#[tokio::test]
async fn escapes_path_values() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/domains/a%2Fb"))
        .and(body_json(json!({ "action": "verify" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true })))
        .expect(1)
        .mount(&server)
        .await;

    client(&server).await.domains().verify("a/b").await.unwrap();
}

#[tokio::test]
async fn removes_a_suppression_by_email() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/suppressions"))
        .and(query_param("email", "a@example.com"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    client(&server)
        .await
        .suppressions()
        .remove("a@example.com")
        .await
        .unwrap();
}
