use serde::{Deserialize, Serialize};

use super::emv::{CardData, EmvTransactionData};

/// Transaction Request from device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    /// Device ID
    pub device_id: String,
    /// Transaction amount (cents)
    pub amount: i64,
    /// Currency code
    pub currency: String,
    /// Card data
    pub card_data: CardData,
    /// EMV transaction data
    pub emv_data: EmvTransactionData,
}

/// Attestation Request to backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationRequest {
    /// Device ID
    pub device_id: String,
    /// Transaction amount
    pub amount: i64,
    /// Currency code
    pub currency_code: String,
    /// Transaction type
    pub transaction_type: String,
    /// Card PAN (masked)
    pub card_pan: String,
    /// Card expiry
    pub card_expiry: String,
    /// Track 2 data
    pub track2_data: Option<String>,
    /// EMV data
    pub emv_data: EmvDataForAttestation,
    /// Client IP
    pub client_ip: Option<String>,
}

/// EMV data for attestation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmvDataForAttestation {
    /// Application ID
    pub aid: String,
    /// Terminal Verification Results
    pub tvr: Option<String>,
    /// Transaction Status Information
    pub tsi: Option<String>,
    /// Application Cryptogram
    pub cryptogram: Option<String>,
    /// Cryptogram Information Data
    pub cid: Option<u8>,
}

/// Attestation Response from backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResponse {
    /// Transaction ID
    pub transaction_id: String,
    /// Status
    pub status: String,
    /// Authorization code
    pub auth_code: Option<String>,
    /// Response message
    pub message: Option<String>,
}

/// Transaction Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Approved,
    Declined,
    Failed,
}
