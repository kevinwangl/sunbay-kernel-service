use crate::models::emv::{ApduCommand, ApduResponse, CardData, Tlv};

/// EMV Processor Service
/// Handles EMV card interaction and APDU command processing
#[derive(Clone)]
pub struct EmvProcessor {
    // Terminal configuration
    terminal_country_code: String,
    terminal_currency_code: String,
}

impl EmvProcessor {
    pub fn new(country_code: String, currency_code: String) -> Self {
        Self {
            terminal_country_code: country_code,
            terminal_currency_code: currency_code,
        }
    }

    /// SELECT PPSE (Payment System Environment)
    pub fn select_ppse(&self) -> ApduCommand {
        // SELECT command: CLA=00, INS=A4, P1=04, P2=00
        // Data: PPSE name "2PAY.SYS.DDF01"
        let ppse_name = b"2PAY.SYS.DDF01";

        ApduCommand::new(0x00, 0xA4, 0x04, 0x00)
            .with_data(ppse_name.to_vec())
            .with_le(0x00)
    }

    /// SELECT Application by AID
    pub fn select_application(&self, aid: &[u8]) -> ApduCommand {
        ApduCommand::new(0x00, 0xA4, 0x04, 0x00)
            .with_data(aid.to_vec())
            .with_le(0x00)
    }

    /// READ RECORD command
    pub fn read_record(&self, sfi: u8, record: u8) -> ApduCommand {
        // P1 = record number, P2 = (SFI << 3) | 0x04
        let p2 = (sfi << 3) | 0x04;

        ApduCommand::new(0x00, 0xB2, record, p2).with_le(0x00)
    }

    /// GET PROCESSING OPTIONS (GPO)
    pub fn get_processing_options(&self, pdol_data: &[u8]) -> ApduCommand {
        // Build PDOL data with tag 0x83
        let mut data = vec![0x83, pdol_data.len() as u8];
        data.extend_from_slice(pdol_data);

        ApduCommand::new(0x80, 0xA8, 0x00, 0x00)
            .with_data(data)
            .with_le(0x00)
    }

    /// GENERATE AC (Application Cryptogram)
    pub fn generate_ac(&self, ac_type: u8, cdol_data: &[u8]) -> ApduCommand {
        // P1: AC type (0x40=AAC, 0x80=TC, 0xC0=ARQC)
        ApduCommand::new(0x80, 0xAE, ac_type, 0x00)
            .with_data(cdol_data.to_vec())
            .with_le(0x00)
    }

    /// Parse card data from TLV response
    pub fn parse_card_data(&self, tlv_data: &[u8], aid: String) -> Result<CardData, String> {
        let tlvs = Tlv::parse(tlv_data)?;

        // Extract PAN (tag 0x5A)
        let pan_tlv = Tlv::find_by_tag(&tlvs, &[0x5A]).ok_or("PAN not found")?;
        let pan = self.format_pan(&pan_tlv.value);

        // Extract expiry date (tag 0x5F24)
        let expiry_tlv = Tlv::find_by_tag(&tlvs, &[0x5F, 0x24]).ok_or("Expiry date not found")?;
        let expiry = self.format_expiry(&expiry_tlv.value);

        // Extract cardholder name (tag 0x5F20) - optional
        let cardholder_name = Tlv::find_by_tag(&tlvs, &[0x5F, 0x20])
            .map(|tlv| String::from_utf8_lossy(&tlv.value).to_string());

        // Extract Track 2 (tag 0x57) - optional
        let track2 = Tlv::find_by_tag(&tlvs, &[0x57]).map(|tlv| hex::encode(&tlv.value));

        // Extract application label (tag 0x50) - optional
        let app_label = Tlv::find_by_tag(&tlvs, &[0x50])
            .map(|tlv| String::from_utf8_lossy(&tlv.value).to_string());

        Ok(CardData {
            pan,
            expiry,
            cardholder_name,
            track2,
            aid,
            app_label,
        })
    }

    /// Format PAN from BCD encoding
    fn format_pan(&self, bcd_data: &[u8]) -> String {
        let hex_str = hex::encode(bcd_data);
        // Remove padding 'F'
        hex_str
            .trim_end_matches('f')
            .trim_end_matches('F')
            .to_string()
    }

    /// Format expiry date from BCD (YYMMDD -> YYMM)
    fn format_expiry(&self, bcd_data: &[u8]) -> String {
        if bcd_data.len() >= 2 {
            format!("{:02x}{:02x}", bcd_data[0], bcd_data[1])
        } else {
            String::new()
        }
    }

    /// Validate APDU response
    pub fn validate_response(&self, response: &ApduResponse) -> Result<(), String> {
        if !response.is_success() {
            return Err(format!("APDU error: SW={:04X}", response.status_word()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_ppse() {
        let processor = EmvProcessor::new("156".to_string(), "CNY".to_string());
        let cmd = processor.select_ppse();

        assert_eq!(cmd.cla, 0x00);
        assert_eq!(cmd.ins, 0xA4);
        assert_eq!(cmd.p1, 0x04);
        assert_eq!(cmd.p2, 0x00);
    }

    #[test]
    fn test_read_record() {
        let processor = EmvProcessor::new("156".to_string(), "CNY".to_string());
        let cmd = processor.read_record(1, 1);

        assert_eq!(cmd.cla, 0x00);
        assert_eq!(cmd.ins, 0xB2);
        assert_eq!(cmd.p1, 1);
        assert_eq!(cmd.p2, 0x0C); // (1 << 3) | 0x04
    }
}
