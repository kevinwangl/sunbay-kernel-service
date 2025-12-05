use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::{
    handlers::AppState,
    models::transaction::{
        AttestationRequest, AttestationResponse, EmvDataForAttestation, TransactionRequest,
    },
};

/// Attest Transaction Handler
///
/// POST /api/transactions/attest
pub async fn attest_transaction(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TransactionRequest>,
) -> Result<impl IntoResponse, String> {
    tracing::info!(
        "Transaction attestation request: device={}, amount={}",
        req.device_id,
        req.amount
    );

    // Build attestation request for backend
    let attestation_req = AttestationRequest {
        device_id: req.device_id.clone(),
        amount: req.amount,
        currency_code: req.currency.clone(),
        transaction_type: format!("{:?}", req.emv_data.transaction_type).to_lowercase(),
        card_pan: req.card_data.pan.clone(),
        card_expiry: req.card_data.expiry.clone(),
        track2_data: req.card_data.track2.clone(),
        emv_data: EmvDataForAttestation {
            aid: req.card_data.aid.clone(),
            tvr: req.emv_data.tvr.clone(),
            tsi: req.emv_data.tsi.clone(),
            cryptogram: req.emv_data.cryptogram.clone(),
            cid: req.emv_data.cid,
        },
        client_ip: None,
    };

    // Forward to backend
    let response = state
        .backend_client
        .attest_transaction(attestation_req)
        .await?;

    tracing::info!(
        "Transaction attested successfully: transaction_id={}",
        response.transaction_id
    );

    Ok((StatusCode::OK, Json(response)))
}

/// Get Transaction Status Handler
///
/// GET /api/transactions/:id/status
pub async fn get_transaction_status(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(transaction_id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, String> {
    tracing::info!("Get transaction status: {}", transaction_id);

    // TODO: Query backend for transaction status
    #[derive(serde::Serialize)]
    struct StatusResponse {
        transaction_id: String,
        status: String,
    }

    let response = StatusResponse {
        transaction_id,
        status: "approved".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
