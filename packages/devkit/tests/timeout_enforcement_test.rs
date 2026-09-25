//! Confirms a slow request is aborted rather than hanging forever.
use std::time::Duration;
use tokio::time::timeout;

async fn slow_operation() -> &'static str {
    tokio::time::sleep(Duration::from_secs(5)).await;
    "done"
}

#[tokio::test]
async fn request_times_out_instead_of_hanging() {
    let result = timeout(Duration::from_millis(100), slow_operation()).await;
    assert!(result.is_err(), "expected the operation to time out");
}
