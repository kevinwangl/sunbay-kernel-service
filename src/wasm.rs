use crate::models::emv::{ApduCommand, ApduResponse};
use crate::services::emv_processor::EmvProcessor;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Initialize the EMV processor for WASM
#[wasm_bindgen]
pub struct WasmEmvProcessor {
    processor: EmvProcessor,
}

#[derive(Serialize, Deserialize)]
pub struct SelectRequest {
    pub aid: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReadRecordRequest {
    pub sfi: u8,
    pub record: u8,
}

#[derive(Serialize, Deserialize)]
pub struct GpoRequest {
    pub pdol_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApduCommandResult {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub data: Option<String>,
    pub le: Option<u8>,
}

impl From<ApduCommand> for ApduCommandResult {
    fn from(cmd: ApduCommand) -> Self {
        Self {
            cla: cmd.cla,
            ins: cmd.ins,
            p1: cmd.p1,
            p2: cmd.p2,
            data: cmd.data.map(hex::encode),
            le: cmd.le,
        }
    }
}

#[wasm_bindgen]
impl WasmEmvProcessor {
    /// Create a new EMV processor
    #[wasm_bindgen(constructor)]
    pub fn new(country_code: String, currency_code: String) -> Self {
        Self {
            processor: EmvProcessor::new(country_code, currency_code),
        }
    }

    /// Select PPSE (Payment System Environment)
    #[wasm_bindgen(js_name = selectPpse)]
    pub fn select_ppse(&self) -> JsValue {
        let cmd = self.processor.select_ppse();
        let result: ApduCommandResult = cmd.into();
        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    /// Select application by AID
    #[wasm_bindgen(js_name = selectApplication)]
    pub fn select_application(&self, aid_hex: String) -> Result<JsValue, JsValue> {
        let aid = hex::decode(&aid_hex)
            .map_err(|e| JsValue::from_str(&format!("Invalid AID hex: {}", e)))?;

        let cmd = self.processor.select_application(&aid);
        let result: ApduCommandResult = cmd.into();
        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    /// Read record from card
    #[wasm_bindgen(js_name = readRecord)]
    pub fn read_record(&self, sfi: u8, record: u8) -> JsValue {
        let cmd = self.processor.read_record(sfi, record);
        let result: ApduCommandResult = cmd.into();
        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    /// Get processing options
    #[wasm_bindgen(js_name = getProcessingOptions)]
    pub fn get_processing_options(&self, pdol_hex: String) -> Result<JsValue, JsValue> {
        let pdol_data = hex::decode(&pdol_hex)
            .map_err(|e| JsValue::from_str(&format!("Invalid PDOL hex: {}", e)))?;

        let cmd = self.processor.get_processing_options(&pdol_data);
        let result: ApduCommandResult = cmd.into();
        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    /// Generate AC (Application Cryptogram)
    #[wasm_bindgen(js_name = generateAc)]
    pub fn generate_ac(&self, ac_type: u8, cdol_hex: String) -> Result<JsValue, JsValue> {
        let cdol_data = hex::decode(&cdol_hex)
            .map_err(|e| JsValue::from_str(&format!("Invalid CDOL hex: {}", e)))?;

        let cmd = self.processor.generate_ac(ac_type, &cdol_data);
        let result: ApduCommandResult = cmd.into();
        Ok(serde_wasm_bindgen::to_value(&result).unwrap())
    }

    /// Parse card data from TLV response
    #[wasm_bindgen(js_name = parseCardData)]
    pub fn parse_card_data(&self, tlv_hex: String, aid: String) -> Result<JsValue, JsValue> {
        let tlv_data = hex::decode(&tlv_hex)
            .map_err(|e| JsValue::from_str(&format!("Invalid TLV hex: {}", e)))?;

        let card_data = self
            .processor
            .parse_card_data(&tlv_data, aid)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(serde_wasm_bindgen::to_value(&card_data).unwrap())
    }
}

/// Get the version of the kernel
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
