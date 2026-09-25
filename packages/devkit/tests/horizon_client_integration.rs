//! Integration test against a mocked Horizon fee_stats endpoint.
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn fetches_fee_stats_from_mocked_horizon() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fee_stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "last_ledger": "100",
            "p50_accepted_fee": "100"
        })))
        .mount(&mock_server)
        .await;

    let resp = reqwest::get(format!("{}/fee_stats", mock_server.uri()))
        .await
        .expect("request should succeed");

    assert!(resp.status().is_success());
}
