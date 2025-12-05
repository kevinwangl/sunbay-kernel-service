use reqwest::Client;
use std::time::Duration;

use crate::models::transaction::{AttestationRequest, AttestationResponse};

/// Backend Client Service
/// Handles communication with sunbay-softpos-backend
#[derive(Clone)]
pub struct BackendClient {
    client: Client,
    backend_url: String,
}

impl BackendClient {
    pub fn new(backend_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            backend_url,
        }
    }

    /// Attest transaction with backend
    pub async fn attest_transaction(
        &self,
        request: AttestationRequest,
    ) -> Result<AttestationResponse, String> {
        let url = format!("{}/api/v1/transactions/attest", self.backend_url);

        tracing::info!("Sending attestation request to backend: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send attestation request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Backend returned error {}: {}", status, error_text));
        }

        let attestation_response: AttestationResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse attestation response: {}", e))?;

        tracing::info!(
            "Attestation successful: transaction_id={}",
            attestation_response.transaction_id
        );

        Ok(attestation_response)
    }

    /// Health check backend
    pub async fn health_check(&self) -> Result<(), String> {
        let url = format!("{}/health/check", self.backend_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to check backend health: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!(
                "Backend health check failed: {}",
                response.status()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_client_creation() {
        let client = BackendClient::new("http://localhost:8080".to_string());
        assert_eq!(client.backend_url, "http://localhost:8080");
    }
}
