use serde::{Deserialize, Serialize};

/// APDU Command structure (ISO 7816-4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApduCommand {
    /// Class byte
    pub cla: u8,
    /// Instruction byte
    pub ins: u8,
    /// Parameter 1
    pub p1: u8,
    /// Parameter 2
    pub p2: u8,
    /// Command data (optional)
    pub data: Option<Vec<u8>>,
    /// Expected response length (Le)
    pub le: Option<u8>,
}

impl ApduCommand {
    /// Create a new APDU command
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            data: None,
            le: None,
        }
    }

    /// Set command data
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Set expected response length
    pub fn with_le(mut self, le: u8) -> Self {
        self.le = Some(le);
        self
    }

    /// Convert to bytes for transmission
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.cla, self.ins, self.p1, self.p2];

        if let Some(ref data) = self.data {
            bytes.push(data.len() as u8); // Lc
            bytes.extend_from_slice(data);
        }

        if let Some(le) = self.le {
            bytes.push(le);
        }

        bytes
    }
}

/// APDU Response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApduResponse {
    /// Response data
    pub data: Vec<u8>,
    /// Status word 1
    pub sw1: u8,
    /// Status word 2
    pub sw2: u8,
}

impl ApduResponse {
    /// Create from response bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 2 {
            return Err("Response too short".to_string());
        }

        let len = bytes.len();
        Ok(Self {
            data: bytes[..len - 2].to_vec(),
            sw1: bytes[len - 2],
            sw2: bytes[len - 1],
        })
    }

    /// Check if response is successful (9000)
    pub fn is_success(&self) -> bool {
        self.sw1 == 0x90 && self.sw2 == 0x00
    }

    /// Get status word as u16
    pub fn status_word(&self) -> u16 {
        ((self.sw1 as u16) << 8) | (self.sw2 as u16)
    }
}

/// EMV Card Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    /// Primary Account Number (masked)
    pub pan: String,
    /// Card expiry date (YYMM)
    pub expiry: String,
    /// Cardholder name (optional)
    pub cardholder_name: Option<String>,
    /// Track 2 equivalent data
    pub track2: Option<String>,
    /// Application ID (AID)
    pub aid: String,
    /// Application label
    pub app_label: Option<String>,
}

/// EMV Transaction Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmvTransactionData {
    /// Amount (in cents)
    pub amount: i64,
    /// Currency code (ISO 4217)
    pub currency_code: String,
    /// Transaction type
    pub transaction_type: TransactionType,
    /// Terminal Verification Results
    pub tvr: Option<String>,
    /// Transaction Status Information
    pub tsi: Option<String>,
    /// Application Cryptogram
    pub cryptogram: Option<String>,
    /// Cryptogram Information Data
    pub cid: Option<u8>,
}

/// Transaction Type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Purchase,
    Withdrawal,
    Refund,
    CashAdvance,
}

/// TLV (Tag-Length-Value) structure
#[derive(Debug, Clone)]
pub struct Tlv {
    pub tag: Vec<u8>,
    pub value: Vec<u8>,
}

impl Tlv {
    /// Parse TLV data
    pub fn parse(data: &[u8]) -> Result<Vec<Tlv>, String> {
        let mut tlvs = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            // Parse tag
            let tag_start = pos;
            let first_byte = data[pos];
            pos += 1;

            // Check if tag is multi-byte
            if (first_byte & 0x1F) == 0x1F {
                while pos < data.len() && (data[pos] & 0x80) == 0x80 {
                    pos += 1;
                }
                if pos < data.len() {
                    pos += 1;
                }
            }

            let tag = data[tag_start..pos].to_vec();

            if pos >= data.len() {
                break;
            }

            // Parse length
            let length_byte = data[pos];
            pos += 1;

            let length = if (length_byte & 0x80) == 0 {
                // Short form
                length_byte as usize
            } else {
                // Long form
                let num_length_bytes = (length_byte & 0x7F) as usize;
                if pos + num_length_bytes > data.len() {
                    return Err("Invalid length encoding".to_string());
                }

                let mut len = 0usize;
                for _ in 0..num_length_bytes {
                    len = (len << 8) | (data[pos] as usize);
                    pos += 1;
                }
                len
            };

            if pos + length > data.len() {
                return Err("Invalid TLV data".to_string());
            }

            let value = data[pos..pos + length].to_vec();
            pos += length;

            tlvs.push(Tlv { tag, value });
        }

        Ok(tlvs)
    }

    /// Find TLV by tag
    pub fn find_by_tag<'a>(tlvs: &'a [Tlv], tag: &[u8]) -> Option<&'a Tlv> {
        tlvs.iter().find(|tlv| tlv.tag == tag)
    }
}
