//! Demonstrates reusing a single reqwest::Client instead of
//! constructing a new one per request.
use once_cell::sync::Lazy;

static SHARED_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .build()
        .expect("failed to build shared reqwest client")
});

pub fn shared_client() -> &'static reqwest::Client {
    &SHARED_CLIENT
}
