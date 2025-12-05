use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    handlers::AppState,
    models::emv::{ApduCommand, ApduResponse},
};

/// EMV Select Request
#[derive(Debug, Deserialize)]
pub struct SelectRequest {
    pub aid: String,
    pub device_id: String,
}

/// EMV Select Response
#[derive(Debug, Serialize)]
pub struct SelectResponse {
    pub success: bool,
    pub fci: String,
    pub sw: String,
}

/// EMV Read Record Request
#[derive(Debug, Deserialize)]
pub struct ReadRecordRequest {
    pub sfi: u8,
    pub record: u8,
    pub device_id: String,
}

/// EMV Read Response
#[derive(Debug, Serialize)]
pub struct ReadResponse {
    pub success: bool,
    pub data: String,
    pub sw: String,
}

/// EMV GPO Request
#[derive(Debug, Deserialize)]
pub struct GpoRequest {
    pub pdol: String,
    pub device_id: String,
}

/// EMV GPO Response
#[derive(Debug, Serialize)]
pub struct GpoResponse {
    pub success: bool,
    pub aip: String,
    pub afl: String,
    pub sw: String,
}

/// SELECT Application Handler
///
/// POST /api/emv/select
pub async fn select_application(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SelectRequest>,
) -> Result<impl IntoResponse, String> {
    tracing::info!("EMV SELECT request for AID: {}", req.aid);

    // Decode AID from hex
    let aid_bytes = hex::decode(&req.aid).map_err(|e| format!("Invalid AID hex: {}", e))?;

    // Build SELECT command
    let cmd = state.emv_processor.select_application(&aid_bytes);

    // TODO: Send command to device/card reader
    // For now, simulate a successful response
    let mock_response = ApduResponse {
        data: vec![
            0x6F, 0x1A, 0x84, 0x0E, 0x31, 0x50, 0x41, 0x59, 0x2E, 0x53, 0x59, 0x53, 0x2E, 0x44,
            0x44, 0x46, 0x30, 0x31, 0xA5, 0x08, 0x88, 0x01, 0x02, 0x5F, 0x2D, 0x02, 0x65, 0x6E,
        ],
        sw1: 0x90,
        sw2: 0x00,
    };

    state.emv_processor.validate_response(&mock_response)?;

    let response = SelectResponse {
        success: true,
        fci: hex::encode(&mock_response.data),
        sw: format!("{:04X}", mock_response.status_word()),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// READ RECORD Handler
///
/// POST /api/emv/read
pub async fn read_record(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ReadRecordRequest>,
) -> Result<impl IntoResponse, String> {
    tracing::info!(
        "EMV READ RECORD request: SFI={}, Record={}",
        req.sfi,
        req.record
    );

    // Build READ RECORD command
    let cmd = state.emv_processor.read_record(req.sfi, req.record);

    // TODO: Send command to device/card reader
    // For now, simulate a successful response
    let mock_response = ApduResponse {
        data: vec![
            0x70, 0x10, 0x5A, 0x08, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56,
        ],
        sw1: 0x90,
        sw2: 0x00,
    };

    state.emv_processor.validate_response(&mock_response)?;

    let response = ReadResponse {
        success: true,
        data: hex::encode(&mock_response.data),
        sw: format!("{:04X}", mock_response.status_word()),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// GET PROCESSING OPTIONS Handler
///
/// POST /api/emv/gpo
pub async fn get_processing_options(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GpoRequest>,
) -> Result<impl IntoResponse, String> {
    tracing::info!("EMV GPO request");

    // Decode PDOL data from hex
    let pdol_bytes = hex::decode(&req.pdol).map_err(|e| format!("Invalid PDOL hex: {}", e))?;

    // Build GPO command
    let cmd = state.emv_processor.get_processing_options(&pdol_bytes);

    // TODO: Send command to device/card reader
    // For now, simulate a successful response
    let mock_response = ApduResponse {
        data: vec![
            0x77, 0x12, 0x82, 0x02, 0x19, 0x80, 0x94, 0x0C, 0x08, 0x01, 0x01, 0x00, 0x10, 0x01,
            0x01, 0x00, 0x18, 0x01, 0x02, 0x00,
        ],
        sw1: 0x90,
        sw2: 0x00,
    };

    state.emv_processor.validate_response(&mock_response)?;

    let response = GpoResponse {
        success: true,
        aip: "1980".to_string(),
        afl: "0801010010010100180102".to_string(),
        sw: format!("{:04X}", mock_response.status_word()),
    };

    Ok((StatusCode::OK, Json(response)))
}
