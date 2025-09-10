use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const DEFAULT_BASE_URL: &str = "https://aethokit.onrender.com/api/";

#[derive(Debug, Error)]
pub enum AethokitError {
    #[error("GAS KEY is required to initialize the SDK")]
    MissingGasKey,
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("unexpected response status: {status} - {body}")]
    UnexpectedStatus { status: StatusCode, body: String },
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Clone)]
pub struct AethokitConfig {
    gas_key: String,
    #[serde(rename = "rpcOrNetwork", skip_serializing_if = "Option::is_none")]
    rpc_or_network: Option<String>
}

/// Rust client for the Aethokit Gas Sponsorship API.
#[derive(Debug, Clone)]
pub struct Aethokit {
    gas_key: String,
    http: Client,
    base_url: Url,
    rpc_or_network: Option<String>,
}

impl Aethokit {
    /// Initialize the SDK
    ///
    /// # Errors
    /// - `MissingGasKey` if `gas_key` is empty
    pub fn new(config: AethokitConfig) -> Result<Self, AethokitError> {
        let key = &config.gas_key;
        if key.trim().is_empty() {
            return Err(AethokitError::MissingGasKey);
        }
        let base_url = Url::parse(DEFAULT_BASE_URL).unwrap();
        Ok(Self {
            gas_key: key.to_string(),
            http: Client::new(),
            base_url,
            rpc_or_network: config.rpc_or_network,
        })
    }

    /// Retrieve the gas address for the gas tank associated with the GAS KEY.
    pub async fn get_gas_address(&self) -> Result<String, AethokitError> {
        let path = "get-gas-address";
        let resp: GasAddressResponse = self
            .make_request::<(), GasAddressResponse>(path, Method::GET, None)
            .await?;
        Ok(resp.gas_address)
    }

    /// Submit a transaction for sponsorship. Returns the transaction hash.
    pub async fn sponsor_tx(
        &self,
        tx: String,
    ) -> Result<String, AethokitError> {
        let path = "sponsor-tx";
        let tx_req = SponsorTxRequest {
            transaction: tx,
            rpc_or_network: self.rpc_or_network.clone(),
        };
        let resp: SponsorTxResponse = self
            .make_request::<SponsorTxRequest, SponsorTxResponse>(
                path,
                Method::POST,
                Some(&tx_req),
            )
            .await?;
        Ok(resp.hash)
    }

    async fn make_request<B: Serialize + ?Sized, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        method: Method,
        body: Option<&B>,
    ) -> Result<R, AethokitError> {
        let url = self.base_url.join(path).expect("valid path join");
        let mut req = self.http
            .request(method, url)
            .header("accept", "application/json")
            .header("x-gas-key", &self.gas_key);

        if let Some(b) = body {
            req = req.json(b);
        }

        let res = req.send().await?;
        let status = res.status();
        let text = res.text().await?;

        if !status.is_success() {
            return Err(AethokitError::UnexpectedStatus { status, body: text });
        }

        let parsed = serde_json::from_str::<R>(&text)?;
        Ok(parsed)
    }
}


#[derive(Debug, Deserialize)]
struct GasAddressResponse {
    #[serde(rename = "gasAddress")]
    gas_address: String,
}

/// Request body for `sponsor_tx`
#[derive(Debug, Serialize, Clone)]
pub struct SponsorTxRequest {
    /// Serialized transaction string
    pub transaction: String,
    /// Optional RPC endpoint or network name
    #[serde(rename = "rpcOrNetwork", skip_serializing_if = "Option::is_none")]
    pub rpc_or_network: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SponsorTxResponse {
    hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that initializing the SDK with an empty string results in a MissingGasKey error.
    #[test]
    fn rejects_empty_key() {
        let cfg = AethokitConfig {
            gas_key: "".to_string(),
            rpc_or_network: None,
        };
        let err = Aethokit::new(cfg).unwrap_err();
        match err {
            AethokitError::MissingGasKey => {},
            other => panic!("expected MissingGasKey, got {other:?}"),
        }
    }
}
