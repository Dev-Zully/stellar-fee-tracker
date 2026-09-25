//! Network selection for Horizon endpoints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Network {
    Testnet,
    Pubnet,
    Custom(String),
}

impl Network {
    pub fn horizon_url(&self) -> String {
        match self {
            Network::Testnet => "https://horizon-testnet.stellar.org".to_string(),
            Network::Pubnet => "https://horizon.stellar.org".to_string(),
            Network::Custom(url) => url.clone(),
        }
    }
}
